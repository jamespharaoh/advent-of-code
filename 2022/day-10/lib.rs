//! Advent of Code 2022: Day 10: Cathode-Ray Tube
//!
//! [https://adventofcode.com/2022/day/10](https://adventofcode.com/2022/day/10)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Cathode-Ray Tube";
	year = 2022;
	day = 10;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
