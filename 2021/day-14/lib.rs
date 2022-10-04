//! Advent of Code 2021: Day 14: Extended Polymerization
//!
//! [https://adventofcode.com/2021/day/14](https://adventofcode.com/2021/day/14)
//!
//! # Input
//!
//! The first line is a string consisting of upper-case letters. After a blank line, there is a
//! list of rules for inserting extra characters between a specific pair of letters, each on its
//! own line, for example `AB -> C` means to insert `C` between `A` and `B`.
//!
//! # Part one
//!
//! Apply the rules ten times. Find the most common letter and the least common letter in the
//! result, and find the difference.
//!
//! # Part two
//!
//! Same as part one, but repeat the process forty times.
//!
//! # Algorithm
//!
//! Instead of tracking the entire string, we simply track adjacent pairs of letters. When
//! inserting characters, we add the sum to the two new pairs created between the left character
//! and the newly inserted, and the newly inserted and the right.
//!
//! At the end, we convert to a character count by counting only the second of each pair. We also
//! have to count the first character, since it is not the second in pair.

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Extended Polymerization";
	year = 2021;
	day = 14;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
