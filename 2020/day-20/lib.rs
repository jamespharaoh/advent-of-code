//! Advent of Code 2020: Day 20: Jurassic Jigsaw
//!
//! [https://adventofcode.com/2020/day/20](https://adventofcode.com/2020/day/20)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Jurassic Jigsaw";
	year = 2020;
	day = 20;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
