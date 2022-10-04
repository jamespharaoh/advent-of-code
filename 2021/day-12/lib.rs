//! Advent of Code 2021: Day 12: Passage Pathing
//!
//! [https://adventofcode.com/2021/day/12](https://adventofcode.com/2021/day/12)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Passage Pathing";
	year = 2021;
	day = 12;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
