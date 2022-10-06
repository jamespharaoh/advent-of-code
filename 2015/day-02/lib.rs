//! Advent of Code 2015: Day 2: I Was Told There Would Be No Math
//!
//! [https://adventofcode.com/2015/day/2](https://adventofcode.com/2015/day/2)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "I Was Told There Would Be No Math";
	year = 2015;
	day = 2;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
