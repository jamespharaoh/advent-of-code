//! Advent of Code 2019: Day 4: Secure Container
//!
//! [https://adventofcode.com/2019/day/04](https://adventofcode.com/2019/day/04)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Secure Container";
	year = 2019;
	day = 4;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
