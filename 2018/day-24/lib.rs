//! Advent of Code 2018: Day 24: Immune System Simulator 20XX
//!
//! [https://adventofcode.com/2018/day/24](https://adventofcode.com/2018/day/24)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Immune System Simulator 20XX";
	year = 2018;
	day = 24;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
