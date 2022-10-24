//! Advent of Code 2021: Day 8: Seven Segment Search
//!
//! [https://adventofcode.com/2021/day/8](https://adventofcode.com/2021/day/8)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_stvec::prelude::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;
mod solver;

puzzle_info! {
	name = "Seven Segment Search";
	year = 2021;
	day = 8;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
