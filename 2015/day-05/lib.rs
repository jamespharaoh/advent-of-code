//! Advent of Code 2015: Day 5: Doesn't He Have Intern-Elves For This?
//!
//! [https://adventofcode.com/2015/day/5](https://adventofcode.com/2015/day/5)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Doesn't He Have Intern-Elves For This?";
	year = 2015;
	day = 5;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
