//! Advent of Code 2021: Day 13: Transparent Origami
//!
//! [https://adventofcode.com/2021/day/13](https://adventofcode.com/2021/day/13)

use aoc_common::*;
use aoc_ocr as ocr;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;
pub mod tool;

puzzle_info! {
	name = "Transparent Origami";
	year = 2021;
	day = 13;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "run"; method = tool::run; ),
	];
}
