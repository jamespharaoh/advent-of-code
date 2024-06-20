//! Advent of Code 2022: Day 25: Full of Hot Air
//!
//! [https://adventofcode.com/2022/day/25](https://adventofcode.com/2022/day/25)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
mod model;

puzzle_info! {
	name = "Full of Hot Air";
	year = 2022;
	day = 25;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
}
