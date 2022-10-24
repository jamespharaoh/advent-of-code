//! Advent of Code 2019: Day 13: Care Package
//!
//! [https://adventofcode.com/2019/day/13](https://adventofcode.com/2019/day/13)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_2019_intcode as intcode;
use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_stvec::prelude::*;

pub mod input;
pub mod logic;
pub mod model;
pub mod run;

puzzle_info! {
	name = "Care Package";
	year = 2019;
	day = 13;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "run"; method = run::run; ),
	];
}
