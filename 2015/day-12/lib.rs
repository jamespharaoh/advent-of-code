//! Advent of Code 2015: Day 12: JSAbacusFramework.io
//!
//! [https://adventofcode.com/2015/day/12](https://adventofcode.com/2015/day/12)
//!
//! # Input
//!
//! A single line of a subset of JSON. Support for arrays, objects, integers and strings (without
//! escapes).
//!
//! # Part one
//!
//! Sum of all numbers in the input.
//!
//! # Part two
//!
//! Sum of all numbers in the input, excluding any objects (and all children) which directly
//! contain the value "red".
//!
//! # Algorithm
//!
//! This is very simple. Data is parsed into a data structure. A recursive function calculates the
//! result for each part.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "JSAbacusFramework.io";
	year = 2015;
	day = 12;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
