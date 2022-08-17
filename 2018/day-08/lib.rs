//! Advent of Code 2018: Day 8: Memory Maneuver
//!
//! [https://adventofcode.com/2018/day/08](https://adventofcode.com/2018/day/08)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Memory Maneuver";
	year = 2018;
	day = 8;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("138", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("66", puzzle.part_two (EXAMPLE));
	}

}
