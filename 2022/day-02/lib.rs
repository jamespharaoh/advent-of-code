//! Advent of Code 2022: Day 2: Rock Paper Scissors
//!
//! [https://adventofcode.com/2022/day/2](https://adventofcode.com/2022/day/2)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Rock Paper Scissors";
	year = 2022;
	day = 2;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
