//! Advent of Code 2019: Day 14: Space Stoichiometry
//!
//! [https://adventofcode.com/2019/day/14](https://adventofcode.com/2019/day/14)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Space Stoichiometry";
	year = 2019;
	day = 14;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
