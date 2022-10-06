//! Advent of Code 2021: Day 25: Sea Cucumber
//!
//! [https://adventofcode.com/2021/day/25](https://adventofcode.com/2021/day/25)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_bitvec::prelude::*;
use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Sea Cucumber";
	year = 2021;
	day = 25;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
}
