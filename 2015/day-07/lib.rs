//! Advent of Code 2015: Day 7: Some Assembly Required
//!
//! [https://adventofcode.com/2015/day/7](https://adventofcode.com/2015/day/7)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_stvec::prelude::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Some Assembly Required";
	year = 2015;
	day = 7;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
