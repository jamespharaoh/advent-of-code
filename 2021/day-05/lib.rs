//! Advent of Code 2021: Day 5: Hydrothermal Venture
//!
//! [https://adventofcode.com/2021/day/5](https://adventofcode.com/2021/day/5)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Hydrothermal Venture";
	year = 2021;
	day = 5;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
