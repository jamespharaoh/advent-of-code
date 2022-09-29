//! Advent of Code 2021: Day 3: Binary Diagnostic
//!
//! [https://adventofcode.com/2021/day/3](https://adventofcode.com/2021/day/3)
//!
//! # Input
//!
//! Readings as binary numbers, one per line.
//!
//! # Part one
//!
//! Find the most common binary digit, `0` or `1`, in each position, and construct a value using
//! those digits in those places. Do the same for the least common, then return their product.
//!
//! # Part two
//!
//! Starting with the most significant digit, keep all readings where this digit has the most
//! common value, or `1` if there is a tie. Repeat with subsequent digits until only one remains.
//! Repeat with the least common value, using `0` for a tie, then return the product of the two
//! outcomes.
//!
//! # Algorithm
//!
//! Just follow the instructions, there's nothing clever here. I have some helper functions, one to
//! iterate bits from the highest to the lowest, and another to check if there are more ones or
//! zeros in a given position, or if there is a tie. For part two I have a function which is used
//! for both calculations, which accepts a function to decide which bit value to keep, instead of
//! duplicating all the logic twice.
//!
//! I cheat a little, converting the numbers to [`u16`] in the parsing phase and discarding the
//! number of digits. Instead I use the most significant set bit in the list of readings to
//! determine the bits to consider. This makes fuzzing easier, and doesn't affect the output for
//! the examples I've seen.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Binary Diagnostic";
	year = 2021;
	day = 3;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
