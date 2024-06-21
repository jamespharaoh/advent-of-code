//! Advent of Code 2023: Day 11: Cosmic Expansion
//!
//! [https://adventofcode.com/2023/day/11](https://adventofcode.com/2023/day/11)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;
mod model;

puzzle_info! {
	name = "Cosmic Expansion";
	year = 2023;
	day = 11;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
