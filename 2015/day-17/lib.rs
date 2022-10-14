//! Advent of Code 2015: Day 17: No Such Thing as Too Much
//!
//! [https://adventofcode.com/2015/day/17](https://adventofcode.com/2015/day/17)
//!
//! # Input
//!
//! Each line is a decimal integer, representing the size of an available container.
//!
//! To run the examples given as tests, we also support a single input parameter giving a target
//! size other than 150:
//!
//! ```text
//! TARGET=25
//! 20
//! 15
//! 10
//! 5
//! 5
//! ```
//!
//! # Part one
//!
//! Work out the number of combinations of containers so that the size adds up to exactly 150. The
//! containers count separately, even if they are the same size.
//!
//! # Part two
//!
//! Work out the minimum number of containers to hold exactly 150, then work out the total number
//! of combinations of that many containers which can hold exactly 150.
//!
//! # Algorithm
//!
//! The [`combos`](logic::combos) function provides an iterator over all combinations of
//! containers which sum to the target size.
//!
//! For part one, we just use [`count`](Iterator::count). For part two, we
//! [`fold`](Iterator::fold) over the combinations, tracking the smallest number of containers so
//! far and the number of combinations with that specific number.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "No Such Thing as Too Much";
	year = 2015;
	day = 17;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
