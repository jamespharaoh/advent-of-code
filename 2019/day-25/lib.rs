//! Advent of Code 2019: Day 25: Cryostasis
//!
//! [https://adventofcode.com/2019/day/25](https://adventofcode.com/2019/day/25)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_2019_intcode as intcode;
use aoc_common::*;

pub mod game;
pub mod input;
pub mod logic;
pub mod model;
pub mod run;

puzzle_info! {
	name = "Cryostasis";
	year = 2019;
	day = 25;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	commands = [
		( name = "run"; method = run::run; ),
	];
}
