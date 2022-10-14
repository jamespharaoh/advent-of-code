//! Advent of Code 2015: Day 19: Medicine for Rudolph
//!
//! [https://adventofcode.com/2015/day/19](https://adventofcode.com/2015/day/19)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_list as list;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Medicine for Rudolph";
	year = 2015;
	day = 19;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_ONE: & [& str] = & [
		"H => HO",
		"H => OH",
		"O => HH",
		"",
		"HOH",
	];

	const EXAMPLE_TWO: & [& str] = & [
		"e => H",
		"e => O",
		"H => HO",
		"H => OH",
		"O => HH",
		"",
		"HOH",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("4", puzzle.part_one (EXAMPLE_ONE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_two (EXAMPLE_TWO));
	}

}
