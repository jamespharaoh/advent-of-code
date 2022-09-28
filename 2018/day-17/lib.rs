//! Advent of Code 2018: Day 17: Reservoir Research
//!
//! [https://adventofcode.com/2018/day/17](https://adventofcode.com/2018/day/17)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Reservoir Research";
	year = 2018;
	day = 17;
	parse = |input_lines| input::Input::parse_from_lines (input_lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"x=495, y=2..7",
		"y=7, x=495..501",
		"x=501, y=3..7",
		"x=498, y=2..4",
		"x=506, y=1..2",
		"x=498, y=10..13",
		"x=504, y=10..13",
		"y=13, x=498..504",
	];

	#[ test ]
	fn test_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("57", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn test_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("29", puzzle.part_two (EXAMPLE));
	}

}
