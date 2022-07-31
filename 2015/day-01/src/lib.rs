//! Advent of Code 2015: Day 1: Not Quite Lisp
//!
//! [https://adventofcode.com/2015/day/1](https://adventofcode.com/2015/day/1)

use aoc_common::*;

puzzle_info! {
	name = "Not Quite Lisp";
	year = 2015;
	day = 1;
	parse = |input| model::parse_input (input [0]);
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

pub mod logic {

	use super::*;
	use model::Input;

	pub fn part_one (input: Input) -> GenResult <i32> {
		Ok (
			input.iter ().copied ().enumerate ()
				.scan (0_i32, |floor, (idx, dir)| {
					* floor += dir.val ();
					Some ((idx, * floor))
				})
				.last ()
				.map (|(_, floor)| floor)
				.unwrap_or (0_i32)
		)
	}

	pub fn part_two (input: Input) -> GenResult <usize> {
		Ok (
			input.iter ().copied ().enumerate ()
				.scan (0_i32, |floor, (idx, dir)| {
					* floor += dir.val ();
					Some ((idx, * floor))
				})
				.filter_map (|(idx, floor)| (floor < 0_i32).then_some (idx + 1))
				.next ()
				.ok_or ("Never visited the basement") ?
		)
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn part_one () -> GenResult <()> {
			use model::Dir::*;
			assert_eq! (3, logic::part_one (vec! [Up, Up, Up]) ?);
			assert_eq! (-1, logic::part_one (vec! [Up, Down, Down]) ?);
			Ok (())
		}

		#[ test ]
		fn part_two () -> GenResult <()> {
			use model::Dir::*;
			assert_eq! (3, logic::part_two (vec! [Up, Down, Down]) ?);
			assert_err! ("Never visited the basement", logic::part_two (vec! [Up, Down]));
			Ok (())
		}

	}

}

pub mod model {

	use super::*;

	pub type Input = Vec <Dir>;

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Dir { Up, Down }

	impl Dir {
		pub fn val (& self) -> i32 {
			match self { Dir::Up => 1_i32, Dir::Down => -1_i32 }
		}
	}

	pub fn parse_input (input: & str) -> GenResult <Input> {
		Ok (
			input.chars ().enumerate ()
				.map (|(ch_idx, ch)| match ch {
					'(' => Ok (Dir::Up),
					')' => Ok (Dir::Down),
					_ => Err (format! ("Invalid character: char {}: {}", ch_idx + 1, ch)),
				})
				.collect::<Result <_, _>> () ?
		)
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn parse_input () {
			assert_err! ("Invalid character: char 3: X", model::parse_input ("()X"));
		}

	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLES_ONE: & [& str] = & [
		"(())",
		"()()",
		"))(((((",
		"())",
		"))(",
		")))",
		")())())",
	];

	const EXAMPLES_TWO: & [& str] = & [
		")",
		"()())",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		let puzzle = puzzle_metadata ();
		assert_eq! ("0", puzzle.part_one (& [EXAMPLES_ONE [0]]) ?);
		assert_eq! ("0", puzzle.part_one (& [EXAMPLES_ONE [1]]) ?);
		assert_eq! ("3", puzzle.part_one (& [EXAMPLES_ONE [2]]) ?);
		assert_eq! ("-1", puzzle.part_one (& [EXAMPLES_ONE [3]]) ?);
		assert_eq! ("-1", puzzle.part_one (& [EXAMPLES_ONE [4]]) ?);
		assert_eq! ("-3", puzzle.part_one (& [EXAMPLES_ONE [5]]) ?);
		assert_eq! ("-3", puzzle.part_one (& [EXAMPLES_ONE [6]]) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		let puzzle = puzzle_metadata ();
		assert_eq! ("1", puzzle.part_two (& [EXAMPLES_TWO [0]]) ?);
		assert_eq! ("5", puzzle.part_two (& [EXAMPLES_TWO [1]]) ?);
		Ok (())
	}

}
