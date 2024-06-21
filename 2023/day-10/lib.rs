//! Advent of Code 2023: Day 10: Pipe Maze
//!
//! [https://adventofcode.com/2023/day/10](https://adventofcode.com/2023/day/10)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;
mod model;

puzzle_info! {
	name = "Pipe Maze";
	year = 2023;
	day = 10;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
