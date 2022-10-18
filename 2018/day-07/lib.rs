//! Advent of Code 2018: Day 7: The Sum of Its Parts
//!
//! [https://adventofcode.com/2018/day/07](https://adventofcode.com/2018/day/07)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "The Sum of Its Parts";
	year = 2018;
	day = 7;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
