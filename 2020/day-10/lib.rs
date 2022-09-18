//! Advent of Code 2020: Day 10: Adapter Array
//!
//! [https://adventofcode.com/2020/day/10](https://adventofcode.com/2020/day/10)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Adapter Array";
	year = 2020;
	day = 10;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
