//! Advent of Code 2020: Day 1: Report Repair
//!
//! [https://adventofcode.com/2020/day/01](https://adventofcode.com/2020/day/01)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Report Repair";
	year = 2020;
	day = 1;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
