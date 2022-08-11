//! Advent of Code 2017: Day 10: Knot Hash
//!
//! [https://adventofcode.com/2017/day/10](https://adventofcode.com/2017/day/10)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Knot Hash";
	year = 2017;
	day = 10;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_ONE: & [& str] = & [
		"STRING_SIZE=5",
		"3,4,1,5",
	];

	const EXAMPLE_TWO: & [& str] = & [
		"STRING_SIZE=96",
		"3,4,1,5",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("12", puzzle.part_one (EXAMPLE_ONE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("451640353711", puzzle.part_two (EXAMPLE_TWO));
	}

}
