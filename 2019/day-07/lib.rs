//! Advent of Code 2019: Day 7: Amplification Circuit
//!
//! [https://adventofcode.com/2019/day/07](https://adventofcode.com/2019/day/07)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_2019_intcode as intcode;
use aoc_common::*;
use aoc_stvec::prelude::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Amplification Circuit";
	year = 2019;
	day = 7;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
