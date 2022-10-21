//! Advent of Code 2020: Day 22: Crab Combat
//!
//! [https://adventofcode.com/2020/day/22](https://adventofcode.com/2020/day/22)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Crab Combat";
	year = 2020;
	day = 22;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
