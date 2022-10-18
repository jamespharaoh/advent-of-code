//! Advent of Code 2018: Day 18: Settlers of The North Pole
//!
//! [https://adventofcode.com/2018/day/18](https://adventofcode.com/2018/day/18)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Settlers of The North Pole";
	year = 2018;
	day = 18;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
