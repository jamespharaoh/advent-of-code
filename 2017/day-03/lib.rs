//! Advent of Code 2017: Day 3: Spiral Memory
//!
//! [https://adventofcode.com/2017/day/3](https://adventofcode.com/2017/day/3)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Spiral Memory";
	year = 2017;
	day = 3;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {
	pub type Coord = i16;
	pub type Pos = aoc_pos::PosRowCol <Coord>;
	pub type Dir = aoc_pos::Dir2d;
	pub type Turn = aoc_pos::Turn2d;
}
