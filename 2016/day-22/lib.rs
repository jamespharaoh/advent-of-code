//! Advent of Code 2016: Day 22: Grid Computing
//!
//! [https://adventofcode.com/2016/day/22](https://adventofcode.com/2016/day/22)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Grid Computing";
	year = 2016;
	day = 22;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {
	pub type Coord = u8;
	pub type Size = u16;
	pub type Pos = aoc_pos::PosXY <Coord>;
}
