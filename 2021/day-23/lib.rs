//! Advent of Code 2021: Day 23: Amphipod
//!
//! [https://adventofcode.com/2021/day/23](https://adventofcode.com/2021/day/23)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_search as search;

mod examples;
pub mod input;
pub mod logic;
pub mod model;
pub mod tools;

puzzle_info! {
	name = "Amphipod";
	year = 2021;
	day = 23;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "run"; method = tools::run; ),
		( name = "internals"; method = tools::internals; ),
	];
}
