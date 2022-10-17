//! Advent of Code 2017: Day 19: A Series of Tubes
//!
//! [https://adventofcode.com/2017/day/19](https://adventofcode.com/2017/day/19)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "A Series of Tubes";
	year = 2017;
	day = 19;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
