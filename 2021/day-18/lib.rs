//! Advent of Code 2021: Day 18: Snailfish
//!
//! [https://adventofcode.com/2021/day/18](https://adventofcode.com/2021/day/18)

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Snailfish";
	year = 2021;
	day = 18;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
