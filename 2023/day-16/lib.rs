//! Advent of Code 2023: Day 16: The Floor Will Be Lava
//!
//! [https://adventofcode.com/2023/day/16](https://adventofcode.com/2023/day/16)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_stvec::prelude::*;

mod examples;
pub mod input;
pub mod logic;
mod model;

puzzle_info! {
	name = "The Floor Will Be Lava";
	year = 2023;
	day = 16;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
