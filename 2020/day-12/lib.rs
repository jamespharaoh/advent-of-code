//! Advent of Code 2020: Day 12: Rain Risk
//!
//! [https://adventofcode.com/2020/day/12](https://adventofcode.com/2020/day/12)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos::GenPosCore as _;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Rain Risk";
	year = 2020;
	day = 12;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
