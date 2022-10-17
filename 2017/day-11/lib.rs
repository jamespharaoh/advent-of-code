//! Advent of Code 2017: Day 11: Hex Ed
//!
//! [https://adventofcode.com/2017/day/11](https://adventofcode.com/2017/day/11)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Hex Ed";
	year = 2017;
	day = 11;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
