//! Advent of Code 2017: Day 25: The Halting Problem
//!
//! [https://adventofcode.com/2017/day/25](https://adventofcode.com/2017/day/25)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "The Halting Problem";
	year = 2017;
	day = 25;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"Begin in state A.",
		"Perform a diagnostic checksum after 6 steps.",
		"",
		"In state A:",
		"  If the current value is 0:",
		"    - Write the value 1.",
		"    - Move one slot to the right.",
		"    - Continue with state B.",
		"  If the current value is 1:",
		"    - Write the value 0.",
		"    - Move one slot to the left.",
		"    - Continue with state B.",
		"",
		"In state B:",
		"  If the current value is 0:",
		"    - Write the value 1.",
		"    - Move one slot to the left.",
		"    - Continue with state A.",
		"  If the current value is 1:",
		"    - Write the value 1.",
		"    - Move one slot to the right.",
		"    - Continue with state A.",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_one (EXAMPLE));
	}

}
