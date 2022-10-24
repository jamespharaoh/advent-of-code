//! Advent of Code 2015: Day 6: Probably a Fire Hazard
//!
//! [https://adventofcode.com/2015/day/6](https://adventofcode.com/2015/day/6)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos as pos;
use aoc_stvec::prelude::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Probably a Fire Hazard";
	year = 2015;
	day = 6;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
