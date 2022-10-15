//! Advent of Code 2016: Day 7: Internet Protocol Version 7
//!
//! [https://adventofcode.com/2016/day/7](https://adventofcode.com/2016/day/7)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Internet Protocol Version 7";
	year = 2016;
	day = 7;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
