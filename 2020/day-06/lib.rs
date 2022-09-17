//! Advent of Code 2020: Day 6: Custom Customs
//!
//! [https://adventofcode.com/2020/day/06](https://adventofcode.com/2020/day/06)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Custom Customs";
	year = 2020;
	day = 6;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
