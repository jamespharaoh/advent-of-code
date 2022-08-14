//! Advent of Code 2017: Day 20: Particle Swarm
//!
//! [https://adventofcode.com/2017/day/20](https://adventofcode.com/2017/day/20)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos as pos;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Particle Swarm";
	year = 2017;
	day = 20;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_ONE: & [& str] = & [
		"p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>",
		"p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>",
	];

	const EXAMPLE_TWO: & [& str] = & [
		"p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>",
		"p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>",
		"p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>",
		"p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("0", puzzle.part_one (EXAMPLE_ONE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("1", puzzle.part_two (EXAMPLE_TWO));
	}

}
