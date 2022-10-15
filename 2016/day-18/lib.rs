//! Advent of Code 2016: Day 18: Like a Rogue
//!
//! [https://adventofcode.com/2016/day/18](https://adventofcode.com/2016/day/18)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Like a Rogue";
	year = 2016;
	day = 18;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"NUM_ROWS_ONE=10",
		"NUM_ROWS_TWO=20",
		".^^.^.^^^^",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("38", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("93", puzzle.part_two (EXAMPLE));
	}

}
