//! Advent of Code 2015: Day 1: Not Quite Lisp
//!
//! [https://adventofcode.com/2015/day/1](https://adventofcode.com/2015/day/1)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Not Quite Lisp";
	year = 2015;
	day = 1;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
