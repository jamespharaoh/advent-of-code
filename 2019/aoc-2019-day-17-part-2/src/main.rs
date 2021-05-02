use intcode::Machine;
use intcode::Mem;
use intcode::RunResult;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Write as _;
use std::fs;
use std::ops::Add;
use std::rc::Rc;

mod intcode;

fn main () {
	let programme_source = fs::read_to_string ("input").unwrap ();
	let programme = intcode::from_str (& programme_source);
	let board_lines = get_board (& programme);
	let (board, start_pos, start_dir) = parse_board (& board_lines);
	let (prog_main, fn_a, fn_b, fn_c) = find_solution (board.clone (), start_pos, start_dir);
	println! ("Main programme: {}", prog_info (& prog_main));
	println! ("Function A: {}", prog_info (& fn_a));
	println! ("Function B: {}", prog_info (& fn_b));
	println! ("Function C: {}", prog_info (& fn_c));
	let mut programme_2 = programme.clone ();
	programme_2 [0] = 2;
	let mut machine = Machine::new (programme_2);
	machine.input_str (& format! ("{}\n", prog_main.join (",")));
	machine.input_str (& format! ("{}\n", fn_a.join (",")));
	machine.input_str (& format! ("{}\n", fn_b.join (",")));
	machine.input_str (& format! ("{}\n", fn_c.join (",")));
	machine.input_str (& format! ("n\n"));
	let result = loop {
		match machine.run () {
			RunResult::Output (ch) if (0 .. 127).contains (& ch) =>
				print! ("{}", ch as u8 as char),
			RunResult::Output (result) if result >= 128 => break result,
			_ => panic! (),
		}
	};
	println! ("Result: {}", result);
}

fn get_board (programme: & Mem) -> Vec <String> {
	let mut machine = Machine::new (programme.clone ());
	let mut result = Vec::new ();
	let mut line = String::new ();
	loop {
		match machine.run () {
			RunResult::Output (10) => {
				if ! line.is_empty () {
					result.push (line);
					line = String::new ();
				}
			},
			RunResult::Output (ch) if (0 .. 127).contains (& ch) =>
				line.push (ch as u8 as char),
			RunResult::Halt => break,
			_ => panic! (),
		}
	}
	result
}

fn parse_board <LineRef: AsRef <str>> (board_lines: & [LineRef]) -> (Board, Pos, Dir) {
	let mut board: HashSet <Pos> = HashSet::new ();
	let mut start_pos = Pos { x: 0, y: 0 };
	let mut start_dir = Dir::Up;
	let mut scan_y: usize = 0;
	for line in board_lines {
		let line = line.as_ref ().trim ();
		if line.len () == 0 { continue }
		let mut scan_x: usize = 0;
		for ch in line.chars () {
			if ch == '#' {
				board.insert (Pos { x: scan_x, y: scan_y });
			}
			if ch == '^' || ch == 'v' || ch == '<' || ch == '>' {
				start_pos = Pos { x: scan_x, y: scan_y };
				start_dir = match ch {
					'^' => Dir::Up,
					'v' => Dir::Down,
					'<' => Dir::Left,
					'>' => Dir::Right,
					_ => unreachable! (),
				};
			}
			scan_x += 1;
		}
		scan_y += 1;
	}
	(board, start_pos, start_dir)
}

fn find_solution (board: Board, start_pos: Pos, start_dir: Dir) -> (Prog, Prog, Prog, Prog) {
	for prog_full in RouteFinder::new (board.clone (), start_pos, start_dir) {
		for fn_a_len in (1 .. prog_full.len () - 2).rev () {
			let mut prog_remain_a = & prog_full [..];
			let fn_a: Prog = prog_remain_a [0 .. fn_a_len].to_vec ();
			if prog_chars (& fn_a) > 20 { continue }
			let mut prog_main: Prog = Vec::new ();
			while prog_starts_with (prog_remain_a, & fn_a) {
				prog_main.push ("A".into ());
				prog_remain_a = & prog_remain_a [fn_a.len () .. ];
			}
			for fn_b_len in (1 .. prog_remain_a.len () - 1).rev () {
				let mut prog_remain_b = prog_remain_a;
				let fn_b: Prog = prog_remain_b [0 .. fn_b_len].to_vec ();
				if prog_chars (& fn_b) > 20 { continue }
				let mut prog_main: Prog = prog_main.clone ();
				loop {
					if prog_starts_with (prog_remain_b, & fn_a) {
						prog_main.push ("A".into ());
						prog_remain_b = & prog_remain_b [fn_a.len () .. ];
					} else if prog_starts_with (prog_remain_b, & fn_b) {
						prog_main.push ("B".into ());
						prog_remain_b = & prog_remain_b [fn_b.len () .. ];
					} else {
						break;
					}
				}
				for fn_c_len in (1 .. prog_remain_b.len ()).rev () {
					let mut prog_remain_c = prog_remain_b;
					let fn_c: Prog = prog_remain_c [0 .. fn_c_len].to_vec ();
					if prog_chars (& fn_c) > 20 { continue }
					let mut prog_main: Prog = prog_main.clone ();
					loop {
						if prog_starts_with (prog_remain_c, & fn_a) {
							prog_main.push ("A".into ());
							prog_remain_c = & prog_remain_c [fn_a.len () .. ];
						} else if prog_starts_with (prog_remain_c, & fn_b) {
							prog_main.push ("B".into ());
							prog_remain_c = & prog_remain_c [fn_b.len () .. ];
						} else if prog_starts_with (prog_remain_c, & fn_c) {
							prog_main.push ("C".into ());
							prog_remain_c = & prog_remain_c [fn_c.len () .. ];
						} else {
							break;
						}
					}
					if prog_remain_c.len () == 0 {
						return (prog_main, fn_a, fn_b, fn_c);
					}
				}
			}
		}
	}
	panic! ("No solution found");
}

type Board = HashSet <Pos>;
type Word = Rc <str>;
type Prog = Vec <Word>;
type ProgRef = [Word];

struct RouteFinder {
	stack: Vec <RouteFinderFrame>,
}

struct RouteFinderFrame {
	prog: Prog,
	board: HashSet <Pos>,
	pos: Pos,
	dir: Dir,
}

impl RouteFinder {
	fn new (board: HashSet <Pos>, pos: Pos, dir: Dir) -> RouteFinder {
		RouteFinder {
			stack: vec! [RouteFinderFrame {
				prog: Vec::new (),
				board,
				pos,
				dir,
			}],
		}
	}
}

impl Iterator for RouteFinder {
	type Item = Prog;
	fn next (& mut self) -> Option <Prog> {
		loop {
			if self.stack.is_empty () { return None }
			let mut frame = self.stack.pop ().unwrap ();
			if frame.board.is_empty () {
				return Some (frame.prog);
			}
			for (dir_str, dir) in [ ("L", frame.dir.left ()), ("R", frame.dir.right ()) ].iter () {
				let dir = * dir;
				let mut pos = frame.pos;
				let mut steps: u64 = 0;
				loop {
					steps += 1;
					pos = pos + dir;
					if ! frame.board.contains (& pos) { break }
					let has_left = frame.board.contains (& (pos + dir.left ()));
					let has_right = frame.board.contains (& (pos + dir.right ()));
					let has_straight = frame.board.contains (& (pos + dir));
					let mut num_dirs: usize = 0;
					if has_left { num_dirs += 1 }
					if has_right { num_dirs += 1 }
					if has_straight { num_dirs += 1 }
					if num_dirs == 2 { panic! () }
					if num_dirs <= 1 {
						frame.board.remove (& pos);
					}
					if (num_dirs == 0 || has_left || has_right) {
						let mut new_prog = frame.prog.clone ();
						new_prog.push (dir_str.to_string ().into ());
						new_prog.push (steps.to_string ().into ());
						self.stack.push (RouteFinderFrame {
							prog: new_prog,
							board: frame.board.clone (),
							pos: pos,
							dir: dir,
						});
					}
				}
			}
		}
	}
}

fn prog_chars (prog: & ProgRef) -> usize {
	prog.iter ().map (
		|word| word.len () + 1,
	).sum::<usize> () - 1
}

fn prog_info (prog: & ProgRef) -> String {
	format! ("{} ({})", prog.join (","), prog_chars (prog))
}

fn prog_starts_with (prog: & ProgRef, prefix: & ProgRef) -> bool {
	if prog.len () < prefix.len () { return false }
	& prog [0 .. prefix.len ()] == prefix
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
enum Dir { Up, Down, Left, Right }

impl Dir {
	fn left (self) -> Dir {
		match self {
			Dir::Up => Dir::Left,
			Dir::Down => Dir::Right,
			Dir::Left => Dir::Down,
			Dir::Right => Dir::Up,
		}
	}
	fn right (self) -> Dir {
		match self {
			Dir::Up => Dir::Right,
			Dir::Down => Dir::Left,
			Dir::Left => Dir::Up,
			Dir::Right => Dir::Down,
		}
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
struct Pos { x: usize, y: usize }

impl Add <Dir> for Pos {
	type Output = Pos;
	fn add (self, dir: Dir) -> Pos {
		match dir {
			Dir::Up => Pos { x: self.x, y: self.y - 1 },
			Dir::Down => Pos { x: self.x, y: self.y + 1 },
			Dir::Left => Pos { x: self.x - 1, y: self.y },
			Dir::Right => Pos { x: self.x + 1, y: self.y },
		}
	}
}
