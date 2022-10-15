//! Advent of Code 2016: Day 11: Radioisotope Thermoelectric Generators
//!
//! [https://adventofcode.com/2016/day/11](https://adventofcode.com/2016/day/11)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;
pub mod tools;

puzzle_info! {
	name = "Radioisotope Thermoelectric Generators";
	year = 2016;
	day = 11;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "corpus-gen"; method = tools::corpus_gen; ),
	];
}
