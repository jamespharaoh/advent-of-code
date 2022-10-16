//! Advent of Code 2017: Day 5: A Maze of Twisty Trampolines, All Alike
//!
//! [https://adventofcode.com/2017/day/5](https://adventofcode.com/2017/day/5)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "A Maze of Twisty Trampolines, All Alike";
	year = 2017;
	day = 5;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {
	pub type Tramp = i16;
}
