//! Advent of Code 2016: Day 2: Bathroom Security
//!
//! [https://adventofcode.com/2016/day/2](https://adventofcode.com/2016/day/2)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Bathroom Security";
	year = 2016;
	day = 2;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
