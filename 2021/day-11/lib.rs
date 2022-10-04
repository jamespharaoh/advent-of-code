//! Advent of Code 2021: Day 11: Dumbo Octopus
//!
//! [https://adventofcode.com/2021/day/11](https://adventofcode.com/2021/day/11)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Dumbo Octopus";
	year = 2021;
	day = 11;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
