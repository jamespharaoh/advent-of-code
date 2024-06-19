//! Advent of Code 2022: Day 22: Monkey Map
//!
//! [https://adventofcode.com/2022/day/22](https://adventofcode.com/2022/day/22)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::*;

mod examples;
pub mod input;
pub mod logic;
mod model;

puzzle_info! {
	name = "Monkey Map";
	year = 2022;
	day = 22;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
