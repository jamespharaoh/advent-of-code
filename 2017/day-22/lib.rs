//! Advent of Code 2017: Day 22: Sporifica Virus
//!
//! [https://adventofcode.com/2017/day/22](https://adventofcode.com/2017/day/22)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos::GenPosCore as _;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Sporifica Virus";
	year = 2017;
	day = 22;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"..#",
		"#..",
		"...",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("5587", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("2511944", puzzle.part_two (EXAMPLE));
	}

}
