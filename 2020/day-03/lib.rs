//! Advent of Code 2020: Day 3: Toboggan Trajectory
//!
//! [https://adventofcode.com/2020/day/03](https://adventofcode.com/2020/day/03)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos::GenPos as _;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Toboggan Trajectory";
	year = 2020;
	day = 3;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
