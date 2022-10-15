//! Advent of Code 2016: Day 13: A Maze of Twisty Little Cubicles
//!
//! [https://adventofcode.com/2016/day/13](https://adventofcode.com/2016/day/13)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "A Maze of Twisty Little Cubicles";
	year = 2016;
	day = 13;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
