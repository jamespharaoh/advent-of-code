//! Advent of Code 2023: Day 23: A Long Walk
//!
//! [https://adventofcode.com/2023/day/23](https://adventofcode.com/2023/day/23)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;
mod model;

puzzle_info! {
	name = "A Long Walk";
	year = 2023;
	day = 23;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
