//! Advent of Code 2021: Day 10: Syntax Scoring
//!
//! [https://adventofcode.com/2021/day/10](https://adventofcode.com/2021/day/10)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Syntax Scoring";
	year = 2021;
	day = 10;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
