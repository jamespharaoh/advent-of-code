//! Advent of Code 2017: Day 11: Hex Ed
//!
//! [https://adventofcode.com/2017/day/11](https://adventofcode.com/2017/day/11)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos as pos;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Hex Ed";
	year = 2017;
	day = 11;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_one (& [ "ne,ne,ne" ]));
		assert_eq_ok! ("0", puzzle.part_one (& [ "ne,ne,sw,sw" ]));
		assert_eq_ok! ("2", puzzle.part_one (& [ "ne,ne,s,s" ]));
		assert_eq_ok! ("3", puzzle.part_one (& [ "se,sw,se,sw,sw" ]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_two (& [ "ne,ne,ne" ]));
		assert_eq_ok! ("2", puzzle.part_two (& [ "ne,ne,sw,sw" ]));
		assert_eq_ok! ("2", puzzle.part_two (& [ "ne,ne,s,s" ]));
		assert_eq_ok! ("3", puzzle.part_two (& [ "se,sw,se,sw,sw" ]));
	}

}
