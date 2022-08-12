//! Advent of Code 2017: Day 13: Packet Scanners
//!
//! [https://adventofcode.com/2017/day/13](https://adventofcode.com/2017/day/13)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Packet Scanners";
	year = 2017;
	day = 13;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [ "0: 3", "1: 2", "4: 4", "6: 4" ];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("24", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("10", puzzle.part_two (EXAMPLE));
	}

}
