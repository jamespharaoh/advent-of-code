//! Advent of Code 2019: Day 10: Monitoring Station
//!
//! [https://adventofcode.com/2019/day/10](https://adventofcode.com/2019/day/10)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid as grid;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Monitoring Station";
	year = 2019;
	day = 10;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
