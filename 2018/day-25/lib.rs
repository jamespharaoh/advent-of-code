//! Advent of Code 2018: Day 25: Four-Dimensional Adventure
//!
//! [https://adventofcode.com/2018/day/25](https://adventofcode.com/2018/day/25)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos as pos;

pub mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Four-Dimensional Adventure";
	year = 2018;
	day = 25;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
}
