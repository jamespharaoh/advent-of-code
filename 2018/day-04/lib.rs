//! Advent of Code 2018: Day 4: Repose Record
//!
//! [https://adventofcode.com/2018/day/04](https://adventofcode.com/2018/day/04)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Repose Record";
	year = 2018;
	day = 4;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
