//! Advent of Code 2023: Day 19: Aplenty
//!
//! [https://adventofcode.com/2023/day/19](https://adventofcode.com/2023/day/19)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
mod model;

puzzle_info! {
	name = "Aplenty";
	year = 2023;
	day = 19;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
