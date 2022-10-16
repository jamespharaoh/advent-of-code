//! Advent of Code 2017: Day 6: Memory Reallocation
//!
//! [https://adventofcode.com/2017/day/6](https://adventofcode.com/2017/day/6)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Memory Reallocation";
	year = 2017;
	day = 6;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {
	pub type Banks = Vec <u8>;
}
