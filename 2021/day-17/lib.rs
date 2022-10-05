//! Advent of Code 2021: Day 17: Trick Shot
//!
//! [https://adventofcode.com/2021/day/17](https://adventofcode.com/2021/day/17)

use aoc_common::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Trick Shot";
	year = 2021;
	day = 17;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
