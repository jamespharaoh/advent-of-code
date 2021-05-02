use intcode::Mem;
use intcode::RunResult;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;
use std::mem;
use std::ops::AddAssign;

mod intcode;

type Prog = Vec <Dir>;

fn main () {
	let programme_source = fs::read_to_string ("input").unwrap ();
	let programme = intcode::from_str (& programme_source);
	let (map, oxygen) = discover_map (& programme);
	print_map (& map, oxygen);
	let num_minutes = fill_map (& map, oxygen);
	println! ("Minutes to fill: {}", num_minutes);
}

fn discover_map (programme: & Mem) -> (HashSet <Pos>, Pos) {

	let mut reached: HashSet <Pos> = HashSet::new ();
	reached.insert (Pos { x: 0, y: 0 });
	let mut probed: HashSet <Pos> = HashSet::new ();
	let mut oxygen: Option <Pos> = None;
	let mut prog: Prog = Vec::new ();
	let mut machine = intcode::Machine::new (programme.clone ());
	let mut pos = Pos { x: 0, y: 0 };

	'OUTER: loop {
		for next_step in vec! [ Dir::North, Dir::South, Dir::West, Dir::East ] {
			let mut next_pos = pos;
			next_pos += next_step;
			if probed.contains (& next_pos) { continue }
			probed.insert (next_pos);
			machine.queue_input (next_step.into ());
			match machine.run () {
				RunResult::Output (value) => match Outcome::try_from (value).unwrap () {
					Outcome::Wall => continue 'OUTER,
					Outcome::Found => oxygen = Some (next_pos),
					_ => (),
				},
				RunResult::Input => panic! ("unexpected input"),
				RunResult::Halt => panic! ("unexpected halt"),
			}
			pos = next_pos;
			prog.push (next_step);
			reached.insert (pos);
			continue 'OUTER;
		}
		if prog.is_empty () { break }
		let last_step = prog.pop ().unwrap ().reverse ();
		machine.queue_input (last_step.into ());
		match machine.run () {
			RunResult::Output (value) => match Outcome::try_from (value).unwrap () {
				Outcome::Wall => panic! (),
				_ => (),
			},
			RunResult::Input => panic! ("unexpected input"),
			RunResult::Halt => panic! ("unexpected halt"),
		}
		pos += last_step;
	}

	(reached, oxygen.unwrap ())

}

fn print_map (map: & HashSet <Pos>, oxygen: Pos) {
	let x_min = map.iter ().map (|pos| pos.x).min ().unwrap ();
	let x_max = map.iter ().map (|pos| pos.x).max ().unwrap ();
	let y_min = map.iter ().map (|pos| pos.y).min ().unwrap ();
	let y_max = map.iter ().map (|pos| pos.y).max ().unwrap ();
	for y in y_min - 1 ..= y_max + 1 {
		for x in x_min - 1 ..= x_max + 1 {
			if x == 0 && y == 0 {
				print! ("St");
			} else if x == oxygen.x && y == oxygen.y {
				print! ("Ox");
			} else if map.contains (& Pos { x, y }) {
				print! ("  ");
			} else {
				print! ("##");
			}
		}
		print! ("\n");
	}
}

fn fill_map (map: & HashSet <Pos>, oxygen: Pos) -> u64 {
	let mut remaining = map.clone ();
	remaining.remove (& oxygen);
	let mut just_filled: HashSet <Pos> = HashSet::new ();
	just_filled.insert (oxygen);
	let mut num_minutes: u64 = 0;
	while ! remaining.is_empty () {
		let mut just_filled_tmp: HashSet <Pos> = HashSet::new ();
		mem::swap (& mut just_filled, & mut just_filled_tmp);
		for pos in just_filled_tmp.iter () {
			for next_step in vec! [ Dir::North, Dir::South, Dir::West, Dir::East ] {
				let mut pos = * pos;
				pos += next_step;
				if ! remaining.contains (& pos) { continue }
				just_filled.insert (pos);
				remaining.remove (& pos);
			}
		}
		num_minutes += 1;
	}
	num_minutes
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
enum Dir { North, South, West, East }

impl Dir {
	fn reverse (self) -> Dir {
		match self {
			Dir::North => Dir::South,
			Dir::South => Dir::North,
			Dir::West => Dir::East,
			Dir::East => Dir::West,
		}
	}
}

impl From <Dir> for i64 {
	fn from (value: Dir) -> i64 {
		match value {
			Dir::North => 1,
			Dir::South => 2,
			Dir::West => 3,
			Dir::East => 4,
		}
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
struct Pos { x: i64, y: i64 }

impl AddAssign <Dir> for Pos {
	fn add_assign (& mut self, dir: Dir) {
		match dir {
			Dir::North => self.y -= 1,
			Dir::South => self.y += 1,
			Dir::West => self.x -= 1,
			Dir::East => self.x += 1,
		}
	}
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
enum Outcome { Wall, Moved, Found }

impl TryFrom <i64> for Outcome {
	type Error = String;
	fn try_from (value: i64) -> Result <Outcome, String> {
		Ok (match value {
			0 => Outcome::Wall,
			1 => Outcome::Moved,
			2 => Outcome::Found,
			_ => return Err (format! ("Invalid outcome: {}", value)),
		})
	}
}
