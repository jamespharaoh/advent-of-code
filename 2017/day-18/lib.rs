//! Advent of Code 2017: Day 18: Duet
//!
//! [https://adventofcode.com/2017/day/18](https://adventofcode.com/2017/day/18)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_2017_cpu as cpu;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Duet";
	year = 2017;
	day = 18;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
