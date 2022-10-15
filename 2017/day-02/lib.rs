//! Advent of Code 2017: Day 2: Corruption Checksum
//!
//! [https://adventofcode.com/2017/day/2](https://adventofcode.com/2017/day/2)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Corruption Checksum";
	year = 2017;
	day = 2;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {
	pub type Value = u16;
}
