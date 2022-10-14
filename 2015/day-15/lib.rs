//! Advent of Code 2015: Day 15: Science for Hungry People
//!
//! [https://adventofcode.com/2015/day/15](https://adventofcode.com/2015/day/15)
//!
//! # Input
//!
//! A list of ingredients in the following format, where `A`-`E` are integers:
//!
//! ```text
//! Name: capacity A, durability B, flavor C, texture D, calories E
//! ```
//!
//! # Part one
//!
//! Find the combination of exactly 100 ingredients with the highest score. The score is
//! calculated by summing the capacity, durability, flavour and texture of each ingredient, then
//! multiplying them together. If any attribute sums to less than zero then the score is zero.
//!
//! # Part two
//!
//! Same as part one, except the total number of calories must be exactly five hundred.
//!
//! # Algorithm
//!
//! Iterate every combination of exactly one hundred ingredients, filter on the number of calories
//! for part two, then find the maximum.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Science for Hungry People";
	year = 2015;
	day = 15;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
