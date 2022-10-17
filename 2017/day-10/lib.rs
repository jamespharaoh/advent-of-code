//! Advent of Code 2017: Day 10: Knot Hash
//!
//! [https://adventofcode.com/2017/day/10](https://adventofcode.com/2017/day/10)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_2017_knot as knot;
use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Knot Hash";
	year = 2017;
	day = 10;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
