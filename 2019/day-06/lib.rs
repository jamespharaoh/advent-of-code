//! Advent of Code 2019: Day 6: Universal Orbit Map
//!
//! [https://adventofcode.com/2019/day/06](https://adventofcode.com/2019/day/06)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Universal Orbit Map";
	year = 2019;
	day = 6;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
