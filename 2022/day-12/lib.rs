//! Advent of Code 2022: Day 12: Hill Climbing Algorithm
//!
//! [https://adventofcode.com/2022/day/12](https://adventofcode.com/2022/day/12)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_search::prelude::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Hill Climbing Algorithm";
	year = 2022;
	day = 12;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
