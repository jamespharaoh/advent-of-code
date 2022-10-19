//! Advent of Code 2019: Day 2: 1202 Program Alarm
//!
//! [https://adventofcode.com/2019/day/02](https://adventofcode.com/2019/day/02)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_2019_intcode as intcode;
use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "1202 Program Alarm";
	year = 2019;
	day = 2;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {
	use super::*;
	pub type Cpu = intcode::Machine <Val>;
	pub type Mem = intcode::Mem <Val>;
	pub type RunResult = intcode::RunResult <Val>;
	pub type Val = i32;
}
