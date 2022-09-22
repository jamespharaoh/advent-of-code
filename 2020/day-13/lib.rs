//! Advent of Code 2020: Day 13: Shuttle Search
//!
//! [https://adventofcode.com/2020/day/13](https://adventofcode.com/2020/day/13)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Shuttle Search";
	year = 2020;
	day = 13;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
