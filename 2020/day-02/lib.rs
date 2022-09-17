//! Advent of Code 2020: Day 2: Password Philosophy
//!
//! [https://adventofcode.com/2020/day/02](https://adventofcode.com/2020/day/02)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Password Philosophy";
	year = 2020;
	day = 2;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
