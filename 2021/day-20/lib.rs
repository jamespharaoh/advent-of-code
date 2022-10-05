//! Advent of Code 2021: Day 20: Trench Map
//!
//! [https://adventofcode.com/2021/day/20](https://adventofcode.com/2021/day/20)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;
pub mod tool;

puzzle_info! {
	name = "Trench Map";
	year = 2021;
	day = 20;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "run"; method = tool::run; ),
	];
}
