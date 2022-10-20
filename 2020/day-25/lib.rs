//! Advent of Code 2020: Day 25: Combo Breaker
//!
//! [https://adventofcode.com/2020/day/25](https://adventofcode.com/2020/day/25)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Combo Breaker";
	year = 2020;
	day = 25;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
}
