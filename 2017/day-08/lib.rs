//! Advent of Code 2017: Day 8: I Heard You Like Registers
//!
//! [https://adventofcode.com/2017/day/8](https://adventofcode.com/2017/day/8)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod cpu;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "I Heard You Like Registers";
	year = 2017;
	day = 8;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
