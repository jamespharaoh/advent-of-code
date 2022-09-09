//! Advent of Code 2019: Day 8: Space Image Format
//!
//! [https://adventofcode.com/2019/day/08](https://adventofcode.com/2019/day/08)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_ocr as ocr;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Space Image Format";
	year = 2019;
	day = 8;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
