//! Advent of Code 2022: Day 4: Camp Cleanup
//!
//! [https://adventofcode.com/2022/day/4](https://adventofcode.com/2022/day/4)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Camp Cleanup";
	year = 2022;
	day = 4;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {

	pub type Val = u8;

}
