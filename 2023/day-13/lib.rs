//! Advent of Code 2023: Day 13: Point of Incidence
//!
//! [https://adventofcode.com/2023/day/13](https://adventofcode.com/2023/day/13)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;
mod model;

puzzle_info! {
	name = "Point of Incidence";
	year = 2023;
	day = 13;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
