//! Advent of Code 2022: Day 15: Beacon Exclusion Zone
//!
//! [https://adventofcode.com/2022/day/15](https://adventofcode.com/2022/day/15)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Beacon Exclusion Zone";
	year = 2022;
	day = 15;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {

	pub type Coord = i32;
	pub type Pos = aoc_pos::PosYX <Coord>;

}
