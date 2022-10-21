//! Advent of Code 2020: Day 15: Rambunctious Recitation
//!
//! [https://adventofcode.com/2020/day/15](https://adventofcode.com/2020/day/15)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Rambunctious Recitation";
	year = 2020;
	day = 15;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
