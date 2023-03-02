//! Advent of Code 2022: Day 13: Distress Signal
//!
//! [https://adventofcode.com/2022/day/13](https://adventofcode.com/2022/day/13)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Distress Signal";
	year = 2022;
	day = 13;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
