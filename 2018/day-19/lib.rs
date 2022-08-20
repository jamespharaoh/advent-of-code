//! Advent of Code 2018: Day 16: Chronal Classification
//!
//! [https://adventofcode.com/2018/day/16](https://adventofcode.com/2018/day/16)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_2018_cpu as cpu;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Go With The Flow";
	year = 2018;
	day = 19;
	parse = |input_lines| input::Input::parse_from_lines (input_lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"#ip 0",
		"seti 5 0 1",
		"seti 6 0 2",
		"addi 0 1 0",
		"addr 1 2 3",
		"setr 1 0 0",
		"seti 8 0 4",
		"seti 9 0 5",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("7", puzzle.part_one (EXAMPLE));
	}

}
