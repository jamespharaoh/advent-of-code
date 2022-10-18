//! Advent of Code 2018: Day 9: Marble Mania
//!
//! [https://adventofcode.com/2018/day/09](https://adventofcode.com/2018/day/09)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Marble Mania";
	year = 2018;
	day = 9;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
