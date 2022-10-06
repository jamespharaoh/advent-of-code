//! Advent of Code 2015: Day 3: Perfectly Spherical Houses in a Vacuum
//!
//! [https://adventofcode.com/2015/day/3](https://adventofcode.com/2015/day/3)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Perfectly Spherical Houses in a Vacuum";
	year = 2015;
	day = 3;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
