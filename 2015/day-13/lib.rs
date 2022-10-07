//! Advent of Code 2015: Day 13: Knights of the Dinner Table
//!
//! [https://adventofcode.com/2015/day/13](https://adventofcode.com/2015/day/13)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_search::prelude::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Knights of the Dinner Table";
	year = 2015;
	day = 13;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
