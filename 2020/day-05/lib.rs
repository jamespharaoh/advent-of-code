//! Advent of Code 2020: Day 5: Binary Boarding
//!
//! [https://adventofcode.com/2020/day/05](https://adventofcode.com/2020/day/05)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Binary Boarding";
	year = 2020;
	day = 5;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
