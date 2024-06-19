//! Advent of Code 2022: Day 23: Unstable Diffusion
//!
//! [https://adventofcode.com/2022/day/23](https://adventofcode.com/2022/day/23)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Unstable Diffusion";
	year = 2022;
	day = 23;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

mod model {
	use super::*;
	use input::Tile;
	pub type Coord = i16;
	pub type Dir = aoc_pos::DirGeo;
	pub type Grid = aoc_grid::GridBuf <Vec <Tile>, Pos, 2>;
	pub type Pos = aoc_pos::PosGeo <Coord>;
}
