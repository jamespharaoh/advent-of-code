//! Advent of Code 2017: Day 24: Electromagnetic Moat
//!
//! [https://adventofcode.com/2017/day/24](https://adventofcode.com/2017/day/24)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Electromagnetic Moat";
	year = 2017;
	day = 24;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [ "0/2", "2/2", "2/3", "3/4", "3/5", "0/1", "10/1", "9/10" ];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("31", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("19", puzzle.part_two (EXAMPLE));
	}

}