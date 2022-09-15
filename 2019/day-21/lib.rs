//! Advent of Code 2019: Day 21: Springdroid Adventure
//!
//! [https://adventofcode.com/2019/day/21](https://adventofcode.com/2019/day/21)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_2019_intcode as intcode;
use aoc_common::*;

pub mod emul;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Springdroid Adventure";
	year = 2019;
	day = 21;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
