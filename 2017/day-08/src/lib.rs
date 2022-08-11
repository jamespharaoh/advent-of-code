//! Advent of Code 2017: Day 8: I Heard You Like Registers
//!
//! [https://adventofcode.com/2017/day/8](https://adventofcode.com/2017/day/8)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod cpu;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "I Heard You Like Registers";
	year = 2017;
	day = 8;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"b inc 5 if a > 1",
		"a inc 1 if b < 5",
		"c dec -10 if a >= 1",
		"c inc -20 if c == 10",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("1", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("10", puzzle.part_two (EXAMPLE));
	}

}
