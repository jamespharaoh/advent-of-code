//! Advent of Code 2017: Day 15: Dueling Generators
//!
//! [https://adventofcode.com/2017/day/15](https://adventofcode.com/2017/day/15)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Dueling Generators";
	year = 2017;
	day = 15;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"Generator A starts with 65",
		"Generator B starts with 8921",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("588", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("309", puzzle.part_two (EXAMPLE));
	}

}
