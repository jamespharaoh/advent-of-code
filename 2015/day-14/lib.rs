//! Advent of Code 2015: Day 14: Reindeer Olympics
//!
//! [https://adventofcode.com/2015/day/14](https://adventofcode.com/2015/day/14)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Reindeer Olympics";
	year = 2015;
	day = 14;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
