use std::collections::HashMap;
use std::fs;
use std::ops::Add;

mod intcode;

fn main () {
	let programme_source = fs::read_to_string ("input").unwrap ();
	let programme = intcode::from_str (& programme_source);
	let mut machine = intcode::Machine::new (programme);
	let mut painted: HashMap <Vec2, Colour> = HashMap::new ();
	let mut pos = Vec2 { x: 0, y: 0 };
	let mut dir = Dir::Up;
	loop {
		machine.queue_input (match painted.get (& pos) {
			Some (Colour::White) => 1,
			_ => 0,
		});
		let colour = match machine.run () {
			intcode::RunResult::Output (0) => Colour::Black,
			intcode::RunResult::Output (1) => Colour::White,
			intcode::RunResult::Output (_) => panic! (),
			intcode::RunResult::Halt => break,
		};
		painted.insert (pos, colour);
		dir = match machine.run () {
			intcode::RunResult::Output (0) => dir.turn_counter_clockwise (),
			intcode::RunResult::Output (1) => dir.turn_clockwise (),
			intcode::RunResult::Output (_) => panic! (),
			intcode::RunResult::Halt => break,
		};
		pos = pos + dir.vec2 ();
	}
	println! ("Number of painted squares: {}", painted.len ());
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
enum Colour { Black, White }

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
struct Vec2 {
	x: i64,
	y: i64,
}

impl Add for Vec2 {
	type Output = Vec2;
	fn add (self, other: Vec2) -> Vec2 {
		Vec2 { x: self.x + other.x, y: self.y + other.y }
	}
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
enum Dir { Up, Down, Left, Right }

impl Dir {

	fn turn_counter_clockwise (self) -> Dir {
		match self {
			Dir::Up => Dir::Left,
			Dir::Down => Dir::Right,
			Dir::Left => Dir::Down,
			Dir::Right => Dir::Up,
		}
	}

	fn turn_clockwise (self) -> Dir {
		match self {
			Dir::Up => Dir::Right,
			Dir::Down => Dir::Left,
			Dir::Left => Dir::Up,
			Dir::Right => Dir::Down,
		}
	}

	fn vec2 (self) -> Vec2 {
		match self {
			Dir::Up => Vec2 { x: 0, y: -1 },
			Dir::Down => Vec2 { x: 0, y: 1 },
			Dir::Left => Vec2 { x: -1, y: 0 },
			Dir::Right => Vec2 { x: 1, y: 0 },
		}
	}

}
