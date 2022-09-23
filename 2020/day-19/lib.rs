//! Advent of Code 2020: Day 19: Monster Messages
//!
//! [https://adventofcode.com/2020/day/19](https://adventofcode.com/2020/day/19)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod matcher;
pub mod model;

puzzle_info! {
	name = "Monster Messages";
	year = 2020;
	day = 19;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
