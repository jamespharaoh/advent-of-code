//! Advent of Code 2019: Day 23: Category Six
//!
//! [https://adventofcode.com/2019/day/23](https://adventofcode.com/2019/day/23)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_2019_intcode as intcode;
use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Category Six";
	year = 2019;
	day = 23;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
