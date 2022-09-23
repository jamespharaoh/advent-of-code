//! Advent of Code 2020: Day 18: Operation Order
//!
//! [https://adventofcode.com/2020/day/18](https://adventofcode.com/2020/day/18)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Operation Order";
	year = 2020;
	day = 18;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
