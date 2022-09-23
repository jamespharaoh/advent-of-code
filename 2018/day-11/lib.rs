//! Advent of Code 2018: Day 11: Chronal Charge
//!
//! [https://adventofcode.com/2018/day/11](https://adventofcode.com/2018/day/11)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Chronal Charge";
	year = 2018;
	day = 11;
	parse = |input_lines| input::Input::parse_from_lines (input_lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("33,45", puzzle.part_one (& [ "18" ]));
		assert_eq_ok! ("21,61", puzzle.part_one (& [ "42" ]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("90,269,16", puzzle.part_two (& [ "18" ]));
		assert_eq_ok! ("232,251,12", puzzle.part_two (& [ "42" ]));
	}

}
