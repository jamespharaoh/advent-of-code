//! Advent of Code 2017: Day 19: A Series of Tubes
//!
//! [https://adventofcode.com/2017/day/19](https://adventofcode.com/2017/day/19)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid as grid;
use aoc_pos as pos;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "A Series of Tubes";
	year = 2017;
	day = 19;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"     |          ",
		"     |  +--+    ",
		"     A  |  C    ",
		" F---|----E|--+ ",
		"     |  |  |  D ",
		"     +B-+  +--+ ",
		"                ",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("ABCDEF", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("38", puzzle.part_two (EXAMPLE));
	}

}
