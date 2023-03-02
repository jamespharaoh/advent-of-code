//! Advent of Code 2022: Day 11: Monkey in the Middle
//!
//! [https://adventofcode.com/2022/day/11](https://adventofcode.com/2022/day/11)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Monkey in the Middle";
	year = 2022;
	day = 11;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {
	pub type Item = u64;
	pub type MonkeyId = u8;
}
