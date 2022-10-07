//! Advent of Code 2015: Day 9: All in a Single Night
//!
//! [https://adventofcode.com/2015/day/9](https://adventofcode.com/2015/day/9)
//!
//! # Input
//!
//! Each line represents the distance between two locations in the form "$0 to $1 = $2". The first
//! two parameters are alphanumeric strings representing the locations, tand the third is a
//! integer representing the distance.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_search::prelude::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "All in a Single Night";
	year = 2015;
	day = 9;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
