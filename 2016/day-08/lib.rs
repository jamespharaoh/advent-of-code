//! Advent of Code 2016: Day 8: Two-Factor Authentication
//!
//! [https://adventofcode.com/2016/day/8](https://adventofcode.com/2016/day/8)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_ocr as ocr;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;
pub mod tools;

puzzle_info! {
	name = "Two-Factor Authentication";
	year = 2016;
	day = 8;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "run"; method = tools::run; ),
	];
}
