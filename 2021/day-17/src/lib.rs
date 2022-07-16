//! Advent of Code 2021: Day 17: Trick Shot
//!
//! [https://adventofcode.com/2021/day/17](https://adventofcode.com/2021/day/17)

use aoc_common::*;

puzzle_info! {
	name = "Trick Shot";
	year = 2021;
	day = 17;
	part_one = |lines| logic::part_one (lines [0]);
	part_two = |lines| logic::part_two (lines [0]);
}

mod logic {

	use super::*;
	use model::Input;
	use model::Position;
	use model::Velocity;

	pub fn part_one (input: & str) -> GenResult <i64> {
		let input = Input::parse (input) ?;
		let results = find_solutions (& input) ?;
		Ok (results.into_iter ().map (|(_, height)| height).max ().unwrap ())
	}

	pub fn part_two (input: & str) -> GenResult <i64> {
		let input = Input::parse (input) ?;
		let results = find_solutions (& input) ?;
		Ok (results.len () as i64)
	}

	fn find_solutions (input: & Input) -> GenResult <Vec <(Velocity, i64)>> {
		let mut results: Vec <(Velocity, i64)> = Vec::new ();
		for x_velocity in 1 ..= * input.target_x.end () {
			for y_velocity in * input.target_y.start () ..= -2 * input.target_y.end () {
				let velocity = Velocity { x: x_velocity, y: y_velocity };
				if let Some (max_height) = simulate (& input, velocity) {
					results.push ((velocity, max_height));
				}
			}
		}
		Ok (results)
	}

	fn simulate (input: & Input, velocity: Velocity) -> Option <i64> {
		let mut position = Position { x: 0, y: 0 };
		let mut velocity = velocity;
		let mut max_height = 0;
		while ! ((velocity.x == 0 && position.x < * input.target_x.start ())
				|| (position.x > * input.target_x.end ())
				|| (velocity.y <= 0 && position.y < * input.target_y.start ())) {
			if input.target_x.contains (& position.x)
					&& input.target_y.contains (& position.y) {
				return Some (max_height);
			}
			position.x += velocity.x;
			if velocity.x > 0 { velocity.x -= 1; }
			position.y += velocity.y;
			velocity.y -= 1;
			if position.y > max_height { max_height = position.y; }
		}
		None
	}

}

mod model {

	use super::*;

	#[ derive (Clone, Debug) ]
	pub struct Input {
		pub target_x: RangeInclusive <i64>,
		pub target_y: RangeInclusive <i64>,
	}

	impl Input {
		pub fn parse (input: & str) -> GenResult <Input> {
			let err_fn = |char_idx| format! ("Invalid input: {}; {}", char_idx + 1, input);
			let mut parser = parser::Parser::new (input, err_fn);
			let x_min = parser.expect ("target area: x=") ?.int () ?;
			let x_max = parser.expect ("..") ?.int () ?;
			let y_min = parser.expect (", y=") ?.int () ?;
			let y_max = parser.expect ("..") ?.int () ?;
			parser.end () ?;
			Ok (Input {
				target_x: x_min ..= x_max,
				target_y: y_min ..= y_max,
			})
		}
	}

	#[ derive (Clone, Copy, Debug) ]
	pub struct Position {
		pub x: i64,
		pub y: i64,
	}

	#[ derive (Clone, Copy, Debug) ]
	pub struct Velocity {
		pub x: i64,
		pub y: i64,
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & str = "target area: x=20..30, y=-10..-5";

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (45, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn test_example () -> GenResult <()> {
		assert_eq! (112, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
