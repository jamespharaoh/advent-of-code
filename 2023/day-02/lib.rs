//! Advent of Code 2023: Day 2: Cube Conundrum
//!
//! [https://adventofcode.com/2023/day/2](https://adventofcode.com/2023/day/2)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Cube Conundrum";
	year = 2023;
	day = 2;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
