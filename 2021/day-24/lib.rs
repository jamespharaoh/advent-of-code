//! Advent of Code 2021: Day 24: Arithmetic Logic Unit
//!
//! [https://adventofcode.com/2021/day/24](https://adventofcode.com/2021/day/24)

#![ allow (clippy::missing_inline_in_public_items) ]
#![ allow (dead_code) ]

use aoc_common::*;
use aoc_stvec::prelude::*;

pub mod input;
pub mod logic;
pub mod machine;
pub mod model;
pub mod quick;
pub mod solver;
pub mod tool;

puzzle_info! {
	name = "Arithmetic Logic Unit";
	year = 2021;
	day = 24;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "all"; method = tool::all; ),
		( name = "machine"; method = tool::machine; ),
		( name = "solver"; method = tool::solver; ),
	];
}
