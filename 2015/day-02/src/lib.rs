//! Advent of Code 2015: Day 2: I Was Told There Would Be No Math
//!
//! [https://adventofcode.com/2015/day/2](https://adventofcode.com/2015/day/2)

use aoc_common::*;

puzzle_info! {
	name = "I Was Told There Would Be No Math";
	year = 2015;
	day = 2;
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

mod logic {

	use super::*;

	pub fn part_one (input: & [& str]) -> GenResult <u32> {
		let input = model::parse_input (input) ?;
		Ok (
			input.iter_copied ().map (|(l, w, h)|
				2*l*w + 2*w*h + 2*h*l + [l*w, w*h, h*l].iter_copied ().min ().unwrap ()
			).sum ()
		)
	}

	pub fn part_two (input: & [& str]) -> GenResult <u32> {
		let input = model::parse_input (input) ?;
		Ok (
			input.iter_copied ().map (|(l, w, h)|
				[ 2*(l+w), 2*(w+h), 2*(h+l) ].iter_copied ().min ().unwrap () + l*w*h
			).sum ()
		)
	}

}

mod model {

	use super::*;
	use parser::Parser;

	pub type Input = Vec <(u32, u32, u32)>;

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		Ok (
			input.iter ().enumerate ().map (|(line_idx, line)| -> GenResult <_> {
				let mut parser = Parser::new (line, |char_idx|
					format! ("Invalid input: line {}: char {}: {}",
						line_idx + 1, char_idx + 1, line));
				Ok ((
					parser.int () ?,
					parser.expect ("x") ?.int () ?,
					parser.expect ("x") ?.int () ?,
				))
			}).collect::<Result::<_, _>> () ?
		)
	}
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [ "2x3x4", "1x1x10" ];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (58, logic::part_one (& EXAMPLE [0 .. 1]) ?);
		assert_eq! (43, logic::part_one (& EXAMPLE [1 .. 2]) ?);
		assert_eq! (101, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (34, logic::part_two (& EXAMPLE [0 .. 1]) ?);
		assert_eq! (14, logic::part_two (& EXAMPLE [1 .. 2]) ?);
		assert_eq! (48, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
