//! Advent of Code 2018: Day 6: Chronal Coordinates
//!
//! [https://adventofcode.com/2018/day/06](https://adventofcode.com/2018/day/06)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Chronal Coordinates";
	year = 2018;
	day = 6;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"DIST_TWO=31",
		"1, 1",
		"1, 6",
		"8, 3",
		"3, 4",
		"5, 5",
		"8, 9",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("17", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("16", puzzle.part_two (EXAMPLE));
	}

}
