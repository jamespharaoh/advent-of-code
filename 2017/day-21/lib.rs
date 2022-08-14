//! Advent of Code 2017: Day 21: Fractal Art
//!
//! [https://adventofcode.com/2017/day/21](https://adventofcode.com/2017/day/21)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Fractal Art";
	year = 2017;
	day = 21;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"ITERS_ONE=2",
		"CHECK_RULES=false",
		"../.# => ##./#../...",
		".#./..#/### => #..#/..../..../#..#",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("12", puzzle.part_one (EXAMPLE));
	}

}
