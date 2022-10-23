//! Advent of Code 2015: Day 4: The Ideal Stocking Stuffer
//!
//! [https://adventofcode.com/2015/day/4](https://adventofcode.com/2015/day/4)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_md5 as md5;
use aoc_parallel::{ self as parallel, prelude::* };
use aoc_search::prelude::*;

pub mod cli;
mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "The Ideal Stocking Stuffer";
	year = 2015;
	day = 4;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "run"; method = cli::run; ),
		( name = "find-test-case"; method = cli::find_test_case; ),
	];
}
