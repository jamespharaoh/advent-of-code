//! Advent of Code 2017: Day 20: Particle Swarm
//!
//! [https://adventofcode.com/2017/day/20](https://adventofcode.com/2017/day/20)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Particle Swarm";
	year = 2017;
	day = 20;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
