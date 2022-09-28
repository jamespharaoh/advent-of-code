//! Advent of Code 2018: Day 13: Mine Cart Madness
//!
//! [https://adventofcode.com/2018/day/13](https://adventofcode.com/2018/day/13)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Mine Cart Madness";
	year = 2018;
	day = 13;
	parse = |input_lines| input::Input::parse_from_lines (input_lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_ONE: & [& str] = & [
		"/->-\\        ",
		"|   |  /----\\",
		"| /-+--+-\\  |",
		"| | |  | v  |",
		"\\-+-/  \\-+--/",
		"  \\------/   ",
	];

	const EXAMPLE_TWO: & [& str] = & [
		"/>-<\\  ",
		"|   |  ",
		"| /<+-\\",
		"| | | v",
		"\\>+</ |",
		"  |   ^",
		"  \\<->/",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("7,3", puzzle.part_one (EXAMPLE_ONE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("6,4", puzzle.part_two (EXAMPLE_TWO));
	}

}
