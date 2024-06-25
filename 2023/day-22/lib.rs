//! Advent of Code 2023: Day 22: Sand Slabs
//!
//! [https://adventofcode.com/2023/day/22](https://adventofcode.com/2023/day/22)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Sand Slabs";
	year = 2023;
	day = 22;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

mod model {
	pub type Coord = i16;
	pub type Pos = aoc_pos::PosXYZ <Coord>;
}
