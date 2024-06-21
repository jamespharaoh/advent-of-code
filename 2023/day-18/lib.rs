//! Advent of Code 2023: Day 18: Lavaduct Lagoon
//!
//! [https://adventofcode.com/2023/day/18](https://adventofcode.com/2023/day/18)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Lavaduct Lagoon";
	year = 2023;
	day = 18;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

mod model {
	pub type Coord = i32;
	pub type Dir = aoc_pos::Dir2d;
	pub type Pos = aoc_pos::PosYX <Coord>;
}
