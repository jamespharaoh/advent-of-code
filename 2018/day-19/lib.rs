//! Advent of Code 2018: Day 19: Go With The Flow
//!
//! [https://adventofcode.com/2018/day/19](https://adventofcode.com/2018/day/19)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_2018_cpu as cpu;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Go With The Flow";
	year = 2018;
	day = 19;
	parse = |input_lines| input::Input::parse_from_lines (input_lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
