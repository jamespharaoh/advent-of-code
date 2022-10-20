//! Advent of Code 2018: Day 10: The Stars Align
//!
//! [https://adventofcode.com/2018/day/10](https://adventofcode.com/2018/day/10)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_ocr as ocr;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;
pub mod tools;

puzzle_info! {
	name = "The Stars Align";
	year = 2018;
	day = 10;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "run"; method = tools::run; ),
	];
}
