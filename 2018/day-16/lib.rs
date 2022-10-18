//! Advent of Code 2018: Day 16: Chronal Classification
//!
//! [https://adventofcode.com/2018/day/16](https://adventofcode.com/2018/day/16)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_2018_cpu as cpu;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Chronal Classification";
	year = 2018;
	day = 16;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
