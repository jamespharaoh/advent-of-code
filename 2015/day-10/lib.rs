//! Advent of Code 2015: Day 10: Elves Look, Elves Say
//!
//! [https://adventofcode.com/2015/day/10](https://adventofcode.com/2015/day/10)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Elves Look, Elves Say";
	year = 2015;
	day = 10;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "internals"; method = cli::internals; ),
		( name = "run"; method = cli::run; ),
		( name = "tracking"; method = tracking::run; ),
		( name = "cycles"; method = cycles::run; ),
	];
}

mod cli;
mod cycles;
mod examples;
pub mod input;
pub mod logic;
pub mod model;
mod tracking;
