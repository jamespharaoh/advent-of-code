//! Advent of Code 2017: Day 4: High-Entropy Passphrases
//!
//! [https://adventofcode.com/2017/day/4](https://adventofcode.com/2017/day/4)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "High-Entropy Passphrases";
	year = 2017;
	day = 4;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_ONE: & [& str] = & [
		"aa bb cc dd ee",
		"aa bb cc dd aa",
		"aa bb cc dd aaa",
	];

	const EXAMPLE_TWO: & [& str] = & [
		"abcde fghij",
		"abcde xyz ecdab",
		"a ab abc abd abf abj",
		"iiii oiii ooii oooi oooo",
		"oiii ioii iioi iiio",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("2", puzzle.part_one (EXAMPLE_ONE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_two (EXAMPLE_TWO));
	}

}
