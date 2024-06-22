//! Advent of Code 2023: Day 20: Pulse Propagation
//!
//! [https://adventofcode.com/2023/day/20](https://adventofcode.com/2023/day/20)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
mod model;

puzzle_info! {
	name = "Pulse Propagation";
	year = 2023;
	day = 20;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
