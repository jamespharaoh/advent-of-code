//! Advent of Code 2015: Day 11: Corporate Policy
//!
//! [https://adventofcode.com/2015/day/11](https://adventofcode.com/2015/day/11)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Corporate Policy";
	year = 2015;
	day = 11;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
