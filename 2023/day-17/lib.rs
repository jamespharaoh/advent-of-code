//! Advent of Code 2023: Day 17: Clumsy Crucible
//!
//! [https://adventofcode.com/2023/day/17](https://adventofcode.com/2023/day/17)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_search::prelude::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Clumsy Crucible";
	year = 2023;
	day = 17;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

mod model {
	pub type Coord = i16;
	pub type Dir = aoc_pos::Dir2d;
	pub type Grid = aoc_grid::GridBuf <Vec <u8>, Pos, 2>;
	pub type Pos = aoc_pos::PosYX <Coord>;
}
