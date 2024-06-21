//! Advent of Code 2023: Day 14: Parabolic Reflector Dish
//!
//! [https://adventofcode.com/2023/day/14](https://adventofcode.com/2023/day/14)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;
mod model;

puzzle_info! {
	name = "Parabolic Reflector Dish";
	year = 2023;
	day = 14;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
