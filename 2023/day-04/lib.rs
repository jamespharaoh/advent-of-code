//! Advent of Code 2023: Day 4: Scratchcards
//!
//! [https://adventofcode.com/2023/day/4](https://adventofcode.com/2023/day/4)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Scratchcards";
	year = 2023;
	day = 4;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
