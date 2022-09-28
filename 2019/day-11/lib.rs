//! Advent of Code 2019: Day 11: Space Police
//!
//! [https://adventofcode.com/2019/day/11](https://adventofcode.com/2019/day/11)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_2019_intcode as intcode;
use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_ocr as ocr;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Space Police";
	year = 2019;
	day = 11;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
