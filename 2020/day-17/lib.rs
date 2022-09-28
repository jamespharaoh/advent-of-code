//! Advent of Code 2020: Day 17: Conway Cubes
//!
//! [https://adventofcode.com/2020/day/17](https://adventofcode.com/2020/day/17)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Conway Cubes";
	year = 2020;
	day = 17;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
