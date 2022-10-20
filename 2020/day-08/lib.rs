//! Advent of Code 2020: Day 8: Handheld Halting
//!
//! [https://adventofcode.com/2020/day/08](https://adventofcode.com/2020/day/08)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Handheld Halting";
	year = 2020;
	day = 8;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
