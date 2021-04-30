use std::char;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fs;
use std::io;
use std::io::Write as _;
use std::mem;
use std::rc::Rc;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ("\n").collect ();
	let steps = calculate_steps (& input_lines);
	println! ("Number of steps: {}", steps);
}

fn calculate_steps <LineRef: AsRef <str>> (lines: & [LineRef]) -> u64 {
	let board = Board::parse (lines);
	println! ("Labyrinth map");
	print_board (& board);
	struct Frame {
		steps: u64,
		board: Board,
	}
	let mut stack: VecDeque <Frame> = VecDeque::new ();
	stack.push_back (Frame { steps: 0, board: board.clone () });
	let mut cache: HashMap <BoardId, u64> = HashMap::new ();
	let mut min_steps: u64 = u64::MAX;
	let mut loops: usize = 0;
	println! ("Searching for optimal solution");
	io::stdout ().flush ().unwrap ();
	while let Some (Frame { steps, board }) = stack.pop_front () {
		if loops > 0 && loops & 0xffff == 0 {
			io::stdout ().flush ().unwrap ();
		}
		loops += 1;
		let board_id = board.id ();
		if let Some (cached_steps) = cache.get (& board_id) {
			if steps >= * cached_steps {
				continue;
			}
		}
		cache.insert (board_id, steps);
		if board.is_complete () {
			if steps < min_steps { min_steps = steps }
			continue;
		}
		for hero in 0 .. 4 {
			let mut visited = HashSet::new ();
			visited.insert (board.heros [hero]);
			let mut current: VecDeque <(Pos, u64)> = VecDeque::new ();
			current.push_back ((board.heros [hero], steps));
			while let Some ((current_pos, base_steps)) = current.pop_front () {
				for (next_pos, extra_steps) in board.paths [& current_pos].iter () {
					let next_pos = next_pos.clone ();
					let next_steps = base_steps + extra_steps;
					if ! board.can_pass (next_pos) { continue }
					if visited.contains (& next_pos) { continue }
					visited.insert (next_pos);
					if board.is_key (next_pos) {
						let mut board = board.clone ();
						board.move_to (hero, next_pos);
						stack.push_back (Frame { steps: next_steps, board });
					} else {
						current.push_back ((next_pos, next_steps));
					}
				}
			}
		}
	}
	if min_steps == u64::MAX { panic! () }
	min_steps
}

fn print_board (board: & Board) {
	for y in 0 .. board.size.y {
		for x in 0 .. board.size.x {
			print! ("{}", board.block ((x, y).into ()));
		}
		print! ("\n");
	}
}

#[ derive (Clone, Copy, Eq, Hash, PartialEq) ]
struct BoardId {
	keys: [bool; 26],
	heros: [Pos; 4],
}

#[ derive (Clone) ]
struct Board {
	size: Pos,
	blocks: Rc <Vec <Block>>,
	paths: Rc <HashMap <Pos, Vec <(Pos, u64)>>>,
	heros: [Pos; 4],
	keys: [bool; 26],
}

impl Board {
	fn parse <LineRef: AsRef <str>> (lines: & [LineRef]) -> Board {

		// parse ascii input and record useful information

		let mut blocks: Vec <Block> = Vec::new ();
		let mut width: u16 = 0;
		let mut hero: Option <Pos> = None;
		let mut keys = [false; 26];
		let mut interesting: HashSet <Pos> = HashSet::new ();
		for (y, line) in lines.iter ().enumerate () {
			let line = line.as_ref ();
			for (x, ch) in line.chars ().enumerate () {
			let pos = (x as u16, y as u16).into ();
				blocks.push (match ch {
					'.' => Block::Empty,
					'#' => Block::Wall,
					'@' => {
						hero = Some (pos);
						interesting.insert (pos);
						Block::Hero
					},
					'a' ..= 'z' => {
						let id = (ch as u8) - ('a' as u8);
						keys [id as usize] = true;
						interesting.insert (pos);
						Block::Key (id)
					},
					'A' ..= 'Z' => {
						let id = (ch as u8) - ('A' as u8);
						interesting.insert (pos);
						Block::Door (id)
					},
					_ => panic! ("Invalid block: {}", ch),
				});
			}
			if width == 0 { width = blocks.len () as u16 }
		}
		let size: Pos = (width, lines.len () as u16).into ();
		let hero = hero.unwrap ();

		// split hero into four

		blocks [hero.up ().left ().to_index (size)] = Block::Hero;
		blocks [hero.up ().to_index (size)] = Block::Wall;
		blocks [hero.up ().right ().to_index (size)] = Block::Hero;
		blocks [hero.left ().to_index (size)] = Block::Wall;
		blocks [hero.to_index (size)] = Block::Wall;
		blocks [hero.right ().to_index (size)] = Block::Wall;
		blocks [hero.down ().left ().to_index (size)] = Block::Hero;
		blocks [hero.down ().to_index (size)] = Block::Wall;
		blocks [hero.down ().right ().to_index (size)] = Block::Hero;
		let heros: [Pos; 4] = [
			hero.up ().left (),
			hero.up ().right (),
			hero.down ().left (),
			hero.down ().right (),
		];
		interesting.remove (& hero);
		interesting.insert (hero.up ().left ());
		interesting.insert (hero.up ().right ());
		interesting.insert (hero.down ().left ());
		interesting.insert (hero.down ().right ());

		// mark irrelevant parts of the map

		let mut deadends: HashSet <Pos> = HashSet::new ();
		loop {
			let mut progress = false;
			for y in 1 .. size.y - 1 {
				for x in 1 .. size.x - 1 {
					let pos: Pos = (x, y).into ();
					if deadends.contains (& pos) { continue }
					match blocks [pos.to_index (size)] {
						Block::Empty => (),
						Block::Door (_) => (),
						_ => continue,
					}
					let mut num_walls: u8 = 0;
					for adj_pos in [
						pos.up (), pos.down (),
						pos.left (), pos.right (),
					].iter ().cloned () {
						match blocks [adj_pos.to_index (size)] {
							Block::DeadEnd => (),
							Block::Wall => (),
							_ => if ! deadends.contains (& adj_pos) { continue },
						}
						num_walls += 1;
					}
					if num_walls < 3 { continue }
					deadends.insert (pos);
					progress = true;
				}
			}
			if ! progress { break }
		}
		for pos in deadends.into_iter () {
			let block_mut = & mut blocks [pos.to_index (size)];
			if * block_mut == Block::Empty {
				* block_mut = Block::DeadEnd;
			}
		}

		// compute paths between interesting parts of the map

		let mut paths: HashMap <Pos, Vec <(Pos, u64)>> = HashMap::new ();
		for start_pos in interesting.iter () {
			let start_pos = start_pos.clone ();
			let mut steps: u64 = 0;
			let mut current: HashSet <Pos> = HashSet::new ();
			current.insert (start_pos);
			let mut visited: HashSet <Pos> = current.clone ();
			let mut dests: Vec <(Pos, u64)> = Vec::new ();
			while ! current.is_empty () {
				steps += 1;
				let mut current_tmp: HashSet <Pos> = HashSet::new ();
				mem::swap (& mut current_tmp, & mut current);
				for step_pos in current_tmp.into_iter () {
					for next_pos in [
						step_pos.up (), step_pos.down (),
						step_pos.left (), step_pos.right (),
					].iter ().cloned () {
						if visited.contains (& next_pos) { continue }
						visited.insert (next_pos);
						let next_index = next_pos.to_index (size);
						if blocks [next_index] == Block::Wall { continue }
						if interesting.contains (& next_pos) {
							dests.push ((next_pos, steps));
						} else {
							current.insert (next_pos);
						}
					}
				}
			}
			paths.insert (start_pos.into (), dests);
		}

		Board {
			size,
			blocks: Rc::new (blocks),
			paths: Rc::new (paths),
			heros,
			keys,
		}

	}
	fn block (& self, pos: Pos) -> Block {
		if self.heros.iter ().any (|hero_pos| * hero_pos == pos) { return Block::Hero }
		match self.blocks [pos.y as usize * self.size.x as usize + pos.x as usize] {
			Block::Hero => Block::Empty,
			other => other,
		}
	}
	fn can_pass (& self, pos: Pos) -> bool {
		match self.block (pos) {
			Block::Wall => false,
			Block::Door (id) => ! self.keys [id as usize],
			_ => true,
		}
	}
	fn is_key (& self, pos: Pos) -> bool {
		match self.block (pos) {
			Block::Key (id) => self.keys [id as usize],
			_ => false,
		}
	}
	fn is_complete (& self) -> bool {
		self.keys.iter ().all (|key_state| ! key_state)
	}
	fn move_to (& mut self, hero: usize, pos: Pos) {
		let id = match self.block (pos) {
			Block::Key (id) => id,
			_ => panic! (),
		};
		self.keys [id as usize] = false;
		self.heros [hero] = pos;
	}
	fn id (& self) -> BoardId {
		BoardId {
			keys: self.keys,
			heros: self.heros,
		}
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
struct Pos { x: u16, y: u16 }

impl Pos {
	fn to_index (self, size: Pos) -> usize { self.y as usize * size.x as usize + self.x as usize }
	fn up (self) -> Pos { Pos { x: self.x, y: self.y - 1 } }
	fn down (self) -> Pos { Pos { x: self.x, y: self.y + 1 } }
	fn left (self) -> Pos { Pos { x: self.x - 1, y: self.y } }
	fn right (self) -> Pos { Pos { x: self.x + 1, y: self.y } }
}

impl From <(u16, u16)> for Pos {
	fn from ((x, y): (u16, u16)) -> Pos { Pos { x, y } }
}

impl Display for Pos {
	fn fmt (& self, formatter: & mut Formatter) -> FmtResult {
		write! (formatter, "({},{})", self.x, self.y) ?;
		Ok (())
	}
}

#[ derive (Clone, Copy, Eq, PartialEq) ]
enum Block { Empty, DeadEnd, Wall, Hero, Door (u8), Key (u8) }

impl Display for Block {
	fn fmt (& self, formatter: & mut Formatter) -> FmtResult {
		match self {
			Block::Empty => write! (formatter, "  ") ?,
			Block::DeadEnd => write! (formatter, "▓▓") ?,
			Block::Wall => write! (formatter, "██") ?,
			Block::Hero => write! (formatter, "🤖") ?,
			Block::Door (id) => write! (
				formatter,
				"\x1b[38;5;0m\x1b[48;5;210m{}\x1b[0m",
				char::from_u32 (* id as u32 + 'Ａ' as u32).unwrap (),
			) ?,
			Block::Key (id) => write! (
				formatter,
				"\x1b[38;5;120m{}\x1b[0m",
				char::from_u32 (* id as u32 + 'ａ' as u32).unwrap (),
			) ?,
		}
		Ok (())
	}
}

#[ test ]
fn test_0 () {
	assert_eq! (32, calculate_steps (& vec! [
		"#############",
		"#DcBa.#.GhKl#",
		"#.###...#I###",
		"#e#d#.@.#j#k#",
		"###C#...###J#",
		"#fEbA.#.FgHi#",
		"#############",
	]));
}
