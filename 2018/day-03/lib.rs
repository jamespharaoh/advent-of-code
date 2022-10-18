//! Advent of Code 2018: Day 3: No Matter How You Slice It
//!
//! [https://adventofcode.com/2018/day/03](https://adventofcode.com/2018/day/03)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "No Matter How You Slice It";
	year = 2018;
	day = 3;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
