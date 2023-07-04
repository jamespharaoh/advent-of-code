//! Advent of Code 2022: Day 18: Boiling Boulders
//!
//! [https://adventofcode.com/2022/day/18](https://adventofcode.com/2022/day/18)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::*;
use aoc_pos::GenPos3;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Boiling Boulders";
	year = 2022;
	day = 18;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

mod model {
	pub type Coord = i8;
	pub type Pos = aoc_pos::PosXYZ <Coord>;
	pub type Grid = aoc_grid::GridBuf <Vec <bool>, Pos, 3>;
}
