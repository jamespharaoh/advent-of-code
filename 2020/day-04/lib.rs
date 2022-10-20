//! Advent of Code 2020: Day 4: Passport Processing
//!
//! [https://adventofcode.com/2020/day/04](https://adventofcode.com/2020/day/04)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Passport Processing";
	year = 2020;
	day = 4;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
