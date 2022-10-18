//! Advent of Code 2018: Day 5: Alchemical Reduction
//!
//! [https://adventofcode.com/2018/day/05](https://adventofcode.com/2018/day/05)
//!
//! # Input
//!
//! A single line containing a string of mixed case ASCII letters.
//!
//! # Part one
//!
//! The string represents a "polymer" where each letter is a "unit". A lowercase letter and its
//! uppercase equivalent represent the same unit "type" with different polarity. Whenever these
//! are adjacent, they "react" and are removed from the string. Apply these rules until there are
//! no adjacent units which can react, and return the number of remaining units.
//!
//! # Part two
//!
//! Remove all units of exactly one type, regardless of polarity, to produce the shortest possible
//! resulting polymer, and return its length.
//!
//! # Algorithm
//!
//! We build up the result string one character at a time, always comparing the last char in the
//! result to the next one in the input. Whenever these form a pair, we skip the next char from
//! the input and remove the last one from the output. In this way, we can produce the result in a
//! single pass.
//!
//! For part two, we first reduce the polymer the same as part one. We then try every letter from
//! `a` to `z`, seeing how much we can further reduce the polymer. This works because the
//! reduction process works the same in whatever order. By first reducing the initial input as
//! much as possible we reduce the amount of work that has to be done for each letter in the
//! second part of the process.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Alchemical Reduction";
	year = 2018;
	day = 5;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
