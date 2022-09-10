//! Advent of Code 2019: Day 12: The N-Body Problem
//!
//! [https://adventofcode.com/2019/day/12](https://adventofcode.com/2019/day/12)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "The N-Body Problem";
	year = 2019;
	day = 12;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
