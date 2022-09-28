//! Advent of Code 2019: Day 24: Planet of Discord
//!
//! [https://adventofcode.com/2019/day/24](https://adventofcode.com/2019/day/24)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Planet of Discord";
	year = 2019;
	day = 24;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
