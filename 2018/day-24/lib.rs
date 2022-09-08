//! Advent of Code 2018: Day 24: Immune System Simulator 20XX
//!
//! [https://adventofcode.com/2018/day/24](https://adventofcode.com/2018/day/24)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Immune System Simulator 20XX";
	year = 2018;
	day = 24;
	parse = |input_lines| input::Input::parse_from_lines (input_lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"Immune System:",
		"17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that \
		does 4507 fire damage at initiative 2",
		"989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with \
		an attack that does 25 slashing damage at initiative 3",
		"",
		"Infection:",
		"801 units each with 4706 hit points (weak to radiation) with an attack that does 116 \
		bludgeoning damage at initiative 1",
		"4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an \
		attack that does 12 slashing damage at initiative 4",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("5216", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("51", puzzle.part_two (EXAMPLE));
	}

}
