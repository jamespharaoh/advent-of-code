//! Advent of Code 2016: Day 1: No Time for a Taxicab
//!
//! [https://adventofcode.com/2016/day/1](https://adventofcode.com/2016/day/1)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "No Time for a Taxicab";
	year = 2016;
	day = 1;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLES: & [& str] = & [
		"R2, L3",
		"R2, R2, R2",
		"R5, L5, R5, R3",
		"R8, R4, R4, R8",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("5", puzzle.part_one (& [EXAMPLES [0]]));
		assert_eq_ok! ("2", puzzle.part_one (& [EXAMPLES [1]]));
		assert_eq_ok! ("12", puzzle.part_one (& [EXAMPLES [2]]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("4", puzzle.part_two (& [EXAMPLES [3]]));
	}

}
