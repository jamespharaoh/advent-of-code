//! Advent of Code 2017: Day 7: Recursive Circus
//!
//! [https://adventofcode.com/2017/day/7](https://adventofcode.com/2017/day/7)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Recursive Circus";
	year = 2017;
	day = 7;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"pbga (66)",
		"xhth (57)",
		"ebii (61)",
		"havc (66)",
		"ktlj (57)",
		"fwft (72) -> ktlj, cntj, xhth",
		"qoyq (66)",
		"padx (45) -> pbga, havc, qoyq",
		"tknk (41) -> ugml, padx, fwft",
		"jptl (61)",
		"ugml (68) -> gyxo, ebii, jptl",
		"gyxo (61)",
		"cntj (57)",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("tknk", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("60", puzzle.part_two (EXAMPLE));
	}

}
