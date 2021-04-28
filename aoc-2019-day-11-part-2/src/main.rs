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
	painted.insert (pos, Colour::White);
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
	let mut min_x = 0;
	let mut max_x = 0;
	let mut min_y = 0;
	let mut max_y = 0;
	for (pos, _) in painted.iter () {
		if pos.x < min_x { min_x = pos.x }
		if pos.x > max_x { max_x = pos.x }
		if pos.y < min_y { min_y = pos.y }
		if pos.y > max_y { max_y = pos.y }
	}
	for y in min_y ..= max_y {
		for x in min_x ..= max_x {
			print! ("{}", match painted.get (& Vec2 { x, y }) {
				Some (Colour::White) => '█',
				_ => ' ',
			});
		}
		print! ("\n");
	}
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
