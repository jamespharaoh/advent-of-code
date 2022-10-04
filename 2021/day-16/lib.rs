//! Advent of Code 2021: Day 15: Packet Decoder
//!
//! [https://adventofcode.com/2021/day/16](https://adventofcode.com/2021/day/16)

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Packet Decoder";
	year = 2021;
	day = 16;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
