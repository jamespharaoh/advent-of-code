//! Advent of Code 2015: Day 18: Like a GIF For Your Yard
//!
//! [https://adventofcode.com/2015/day/18](https://adventofcode.com/2015/day/18)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Like a GIF For Your Yard";
	year = 2015;
	day = 18;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
