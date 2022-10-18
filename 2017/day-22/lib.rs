//! Advent of Code 2017: Day 22: Sporifica Virus
//!
//! [https://adventofcode.com/2017/day/22](https://adventofcode.com/2017/day/22)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Sporifica Virus";
	year = 2017;
	day = 22;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
