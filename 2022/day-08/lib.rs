//! Advent of Code 2022: Day 8: Treetop Tree House
//!
//! [https://adventofcode.com/2022/day/8](https://adventofcode.com/2022/day/8)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Treetop Tree House";
	year = 2022;
	day = 8;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {
	pub type Coord = i8;
	pub type Dir = aoc_pos::Dir2d;
	pub type Grid <Cell> = aoc_grid::GridBuf <Vec <Cell>, Pos, 2>;
	pub type Pos = aoc_pos::PosYX <Coord>;
	pub type Val = i8;
}
