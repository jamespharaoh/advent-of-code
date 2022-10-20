//! Advent of Code 2020: Day 24: Lobby Layout
//!
//! [https://adventofcode.com/2020/day/24](https://adventofcode.com/2020/day/24)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Lobby Layout";
	year = 2020;
	day = 24;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
