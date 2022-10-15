//! Advent of Code 2016: Day 23: Safe Cracking
//!
//! [https://adventofcode.com/2016/day/23](https://adventofcode.com/2016/day/23)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_2016_cpu as cpu;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Safe Cracking";
	year = 2016;
	day = 23;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
