//! Advent of Code 2020: Day 7: Handy Haversacks
//!
//! [https://adventofcode.com/2020/day/07](https://adventofcode.com/2020/day/07)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Handy Haversacks";
	year = 2020;
	day = 7;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
