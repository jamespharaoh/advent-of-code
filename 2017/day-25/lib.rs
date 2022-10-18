//! Advent of Code 2017: Day 25: The Halting Problem
//!
//! [https://adventofcode.com/2017/day/25](https://adventofcode.com/2017/day/25)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "The Halting Problem";
	year = 2017;
	day = 25;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
}
