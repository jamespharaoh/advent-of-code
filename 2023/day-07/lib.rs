//! Advent of Code 2023: Day 7: Camel Cards
//!
//! [https://adventofcode.com/2023/day/7](https://adventofcode.com/2023/day/7)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Camel Cards";
	year = 2023;
	day = 7;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
