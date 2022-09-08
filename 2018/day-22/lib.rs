//! Advent of Code 2018: Day 22: Mode Maze
//!
//! [https://adventofcode.com/2018/day/22](https://adventofcode.com/2018/day/22)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid as grid;
use aoc_pos as pos;
use aoc_search as search;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Mode Maze";
	year = 2018;
	day = 22;
	parse = |input_lines| input::Input::parse_from_lines (input_lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"depth: 510",
		"target: 10,10",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("114", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("45", puzzle.part_two (EXAMPLE));
	}

}
