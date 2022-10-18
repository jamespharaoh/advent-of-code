//! Advent of Code 2017: Day 24: Electromagnetic Moat
//!
//! [https://adventofcode.com/2017/day/24](https://adventofcode.com/2017/day/24)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Electromagnetic Moat";
	year = 2017;
	day = 24;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {
	pub type Components = u64;
	pub type Port = u16;
}
