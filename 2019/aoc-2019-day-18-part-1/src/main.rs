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
	stack.push_back (Frame {
		steps: 0,
		board: board.clone (),
	});
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
		let board = board;
		let mut visited = HashSet::new ();
		visited.insert (board.hero);
		let mut current: VecDeque <(Pos, u64)> = VecDeque::new ();
		current.push_back ((board.hero, steps));
		while let Some ((current_pos, base_steps)) = current.pop_front () {
			for (next_pos, extra_steps) in board.paths [& current_pos].iter () {
				let next_pos = next_pos.clone ();
				let next_steps = base_steps + extra_steps;
				if ! board.can_pass (next_pos) { continue }
				if visited.contains (& next_pos) { continue }
				visited.insert (next_pos);
				if board.is_key (next_pos) {
					let mut board = board.clone ();
					board.move_to (next_pos);
					stack.push_back (Frame { steps: next_steps, board });
				} else {
					current.push_back ((next_pos, next_steps));
				}
			}
		}
	}
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
	hero: Pos,
}

#[ derive (Clone) ]
struct Board {
	size: Pos,
	blocks: Rc <Vec <Block>>,
	paths: Rc <HashMap <Pos, Vec <(Pos, u64)>>>,
	hero: Pos,
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
						Pos { x: step_pos.x, y: step_pos.y - 1 },
						Pos { x: step_pos.x, y: step_pos.y + 1 },
						Pos { x: step_pos.x - 1, y: step_pos.y },
						Pos { x: step_pos.x + 1, y: step_pos.y },
					].iter ().cloned () {
						if visited.contains (& next_pos) { continue }
						visited.insert (next_pos);
						let next_index = next_pos.y as usize * size.x as usize + next_pos.x as usize;
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
			hero: hero.unwrap (),
			keys,
		}

	}
	fn block (& self, pos: Pos) -> Block {
		self.blocks [pos.y as usize * self.size.x as usize + pos.x as usize]
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
	fn move_to (& mut self, pos: Pos) {
		let id = match self.block (pos) {
			Block::Key (id) => id,
			_ => panic! (),
		};
		self.keys [id as usize] = false;
		self.hero = pos;
	}
	fn id (& self) -> BoardId {
		BoardId {
			keys: self.keys,
			hero: self.hero,
		}
	}
}

#[ derive (Clone, Copy, Eq, Hash, PartialEq) ]
struct Pos { x: u16, y: u16 }

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
enum Block { Empty, Wall, Hero, Door (u8), Key (u8) }

impl Display for Block {
	fn fmt (& self, formatter: & mut Formatter) -> FmtResult {
		match self {
			Block::Empty => write! (formatter, "  ") ?,
			Block::Wall => write! (formatter, "██") ?,
			Block::Hero => write! (formatter, "👦") ?,
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
	assert_eq! (8, calculate_steps (& vec! [
		"#########",
		"#b.A.@.a#",
		"#########",
	]));
}

#[ test ]
fn test_1 () {
	assert_eq! (132, calculate_steps (& vec! [
		"########################",
		"#...............b.C.D.f#",
		"#.######################",
		"#.....@.a.B.c.d.A.e.F.g#",
		"########################",
	]));
}

#[ test ]
fn test_2 () {
	assert_eq! (136, calculate_steps (& vec! [
		"#################",
		"#i.G..c...e..H.p#",
		"########.########",
		"#j.A..b...f..D.o#",
		"########@########",
		"#k.E..a...g..B.n#",
		"########.########",
		"#l.F..d...h..C.m#",
		"#################",
	]));
}

#[ test ]
fn test_3 () {
	assert_eq! (81, calculate_steps (& vec! [
		"########################",
		"#@..............ac.GI.b#",
		"###d#e#f################",
		"###A#B#C################",
		"###g#h#i################",
		"########################",
	]));
}
