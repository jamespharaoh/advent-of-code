//! Advent of Code 2018: Day 11: Chronal Charge
//!
//! [https://adventofcode.com/2018/day/11](https://adventofcode.com/2018/day/11)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Chronal Charge";
	year = 2018;
	day = 11;
	parse = |input_lines| input::Input::parse_from_lines (input_lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {
	use super::*;
	pub type Coord = i16;
	pub type Grid = GridBuf <Vec <Power>, Pos, 2>;
	pub type Power = i32;
	pub type Pos = aoc_pos::PosYX <Coord>;
}
