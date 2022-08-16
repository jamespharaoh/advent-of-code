//! Advent of Code 2018: Day 5: Alchemical Reduction
//!
//! [https://adventofcode.com/2018/day/05](https://adventofcode.com/2018/day/05)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Alchemical Reduction";
	year = 2018;
	day = 5;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [ "dabAcCaCBAcCcaDA" ];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("10", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("4", puzzle.part_two (EXAMPLE));
	}

}
