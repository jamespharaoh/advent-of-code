//! Advent of Code 2018: Day 6: Chronal Coordinates
//!
//! [https://adventofcode.com/2018/day/06](https://adventofcode.com/2018/day/06)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Chronal Coordinates";
	year = 2018;
	day = 6;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {
	pub type Coord = i16;
	pub type Pos = aoc_pos::PosYX <Coord>;
}
