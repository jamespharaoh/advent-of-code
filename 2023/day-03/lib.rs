//! Advent of Code 2023: Day 3: Gear Ratios
//!
//! [https://adventofcode.com/2023/day/3](https://adventofcode.com/2023/day/3)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Gear Ratios";
	year = 2023;
	day = 3;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
