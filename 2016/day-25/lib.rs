//! Advent of Code 2016: Day 25: Clock Signal
//!
//! [https://adventofcode.com/2016/day/24](https://adventofcode.com/2016/day/24)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_2016_cpu as cpu;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Clock Signal";
	year = 2016;
	day = 25;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
}
