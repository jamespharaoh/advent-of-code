//! Advent of Code 2021: Day 7: The Treachery of Whales
//!
//! [https://adventofcode.com/2021/day/7](https://adventofcode.com/2021/day/7)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod logic;
pub mod input;

puzzle_info! {
	name = "The Treachery of Whales";
	year = 2021;
	day = 7;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
