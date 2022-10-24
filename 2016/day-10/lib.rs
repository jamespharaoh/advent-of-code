//! Advent of Code 2016: Day 10: Balance Bots
//!
//! [https://adventofcode.com/2016/day/10](https://adventofcode.com/2016/day/10)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_stvec::prelude::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Balance Bots";
	year = 2016;
	day = 10;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
