//! Advent of Code 2022: Day 5: Supply Stacks
//!
//! [https://adventofcode.com/2022/day/5](https://adventofcode.com/2022/day/5)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Supply Stacks";
	year = 2022;
	day = 5;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
