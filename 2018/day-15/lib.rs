//! Advent of Code 2018: Day 15: Beverage Bandits
//!
//! [https://adventofcode.com/2018/day/15](https://adventofcode.com/2018/day/15)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;
pub mod state;

puzzle_info! {
	name = "Beverage Bandits";
	year = 2018;
	day = 15;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
