//! Advent of Code 2017: Day 18: Duet
//!
//! [https://adventofcode.com/2017/day/18](https://adventofcode.com/2017/day/18)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_2017_cpu as cpu;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Duet";
	year = 2017;
	day = 18;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_ONE: & [& str] = & [
		"set a 1",
		"add a 2",
		"mul a a",
		"mod a 5",
		"snd a",
		"set a 0",
		"rcv a",
		"jgz a -1",
		"set a 1",
		"jgz a -2",
	];

	const EXAMPLE_TWO: & [& str] = & [
		"snd 1",
		"snd 2",
		"snd p",
		"rcv a",
		"rcv b",
		"rcv c",
		"rcv d",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("4", puzzle.part_one (EXAMPLE_ONE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_two (EXAMPLE_TWO));
	}

}
