//! Advent of Code 2021: Day 21: Dirac Dice
//!
//! [https://adventofcode.com/2021/day/21](https://adventofcode.com/2021/day/21)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Dirac Dice";
	year = 2021;
	day = 21;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
