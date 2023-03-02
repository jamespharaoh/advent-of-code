//! Advent of Code 2022: Day 9: Rope Bridge
//!
//! [https://adventofcode.com/2022/day/9](https://adventofcode.com/2022/day/9)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Rope Bridge";
	year = 2022;
	day = 9;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {
	pub type Coord = i16;
	pub type Dir = aoc_pos::Dir2d;
	pub type Pos = aoc_pos::PosYX <Coord>;
}
