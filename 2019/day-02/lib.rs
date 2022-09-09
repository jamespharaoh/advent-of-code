//! Advent of Code 2019: Day 2: 1202 Program Alarm
//!
//! [https://adventofcode.com/2019/day/02](https://adventofcode.com/2019/day/02)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_2019_intcode as intcode;
use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "1202 Program Alarm";
	year = 2019;
	day = 2;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
