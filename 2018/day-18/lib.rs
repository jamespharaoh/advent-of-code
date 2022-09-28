//! Advent of Code 2018: Day 18: Settlers of The North Pole
//!
//! [https://adventofcode.com/2018/day/18](https://adventofcode.com/2018/day/18)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Settlers of The North Pole";
	year = 2018;
	day = 18;
	parse = |input_lines| input::Input::parse_from_lines (input_lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		".#.#...|#.",
		".....#|##|",
		".|..|...#.",
		"..|#.....#",
		"#.#|||#|#|",
		"...#.||...",
		".|....|...",
		"||...#|.#|",
		"|.||||..|.",
		"...#.|..|.",
	];

	#[ test ]
	fn test_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("1147", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn test_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("0", puzzle.part_two (EXAMPLE));
	}

}
