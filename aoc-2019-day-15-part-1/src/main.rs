use intcode::Mem;
use intcode::RunResult;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;
use std::iter;
use std::mem;
use std::ops::AddAssign;

mod intcode;

type Prog = Vec <Dir>;

fn main () {
	let programme_source = fs::read_to_string ("input").unwrap ();
	let programme = intcode::from_str (& programme_source);
	let route = find_route (& programme);
	println! ("Number of moves: {}", route.len ());
}

fn find_route (programme: & Mem) -> Prog {

	let mut reached: HashSet <Pos> = HashSet::new ();
	let mut progs: Vec <Prog> = Vec::new ();

	progs.push (Vec::new ());

	loop {
		let mut progs_tmp = Vec::new ();
		mem::swap (& mut progs, & mut progs_tmp);
		for prog in progs_tmp.into_iter () {
			'PROG: for next_step in vec! [ Dir::North, Dir::South, Dir::West, Dir::East ] {
				let prog: Prog = prog.iter ().cloned ().chain (iter::once (next_step)).collect ();
				let mut machine = intcode::Machine::new (programme.clone ());
				let mut pos = Pos { x: 0, y: 0 };
				for step in prog.iter () {
					machine.queue_input (step.into ());
					match machine.run () {
						RunResult::Output (value) => match Outcome::try_from (value).unwrap () {
							Outcome::Wall => {
								continue 'PROG;
							},
							Outcome::Moved => pos += step,
							Outcome::Found => return prog,
						},
						RunResult::Input => panic! ("unexpected input"),
						RunResult::Halt => panic! ("unexpected halt"),
					}
				}
				if reached.contains (& pos) {
					continue;
				}
				reached.insert (pos);
				progs.push (prog);
			}
		}
	}

}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
enum Dir { North, South, West, East }

impl From <& Dir> for i64 {
	fn from (value: & Dir) -> i64 {
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

impl AddAssign <& Dir> for Pos {
	fn add_assign (& mut self, dir: & Dir) {
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
