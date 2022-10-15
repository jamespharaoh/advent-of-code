//! Advent of Code 2016: Day 14: One-Time Pad
//!
//! [https://adventofcode.com/2016/day/14](https://adventofcode.com/2016/day/14)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_md5 as md5;
use aoc_parallel as parallel;
use parallel::ThreadMap;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "One-Time Pad";
	year = 2016;
	day = 14;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
