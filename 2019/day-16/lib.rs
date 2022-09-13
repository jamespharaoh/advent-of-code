//! Advent of Code 2019: Day 16: Flawed Frequency Transmission
//!
//! [https://adventofcode.com/2019/day/16](https://adventofcode.com/2019/day/16)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Flawed Frequency Transmission";
	year = 2019;
	day = 16;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
