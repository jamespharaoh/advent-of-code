//! Advent of Code 2015: Day 20: Infinite Elves and Infinite Houses
//!
//! [https://adventofcode.com/2015/day/20](https://adventofcode.com/2015/day/20)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

pub mod model {
	pub type Val = u32;
}

puzzle_info! {
	name = "Infinite Elves and Infinite Houses";
	year = 2015;
	day = 20;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
