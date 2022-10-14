//! Advent of Code 2016: Day 3: Squares With Three Sides
//!
//! [https://adventofcode.com/2016/day/3](https://adventofcode.com/2016/day/3)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Squares With Three Sides";
	year = 2016;
	day = 3;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
