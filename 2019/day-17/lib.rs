//! Advent of Code 2019: Day 17: Set and Forget
//!
//! [https://adventofcode.com/2019/day/17](https://adventofcode.com/2019/day/17)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_2019_intcode as intcode;
use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Set and Forget";
	year = 2019;
	day = 17;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
