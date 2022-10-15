//! Advent of Code 2016: Day 9: Explosives in Cyberspace
//!
//! [https://adventofcode.com/2016/day/9](https://adventofcode.com/2016/day/9)
//!
//! # Input
//!
//! A stream of capital letters or repetition markers in the form `(2x3)`, where `2` is the length
//! and `3` is the number of repetitions.
//!
//! # Part one
//!
//! Determine the uncompressed length, assuming the repetitions apply to the compressed stream
//! directly, without recursively expanding nested repetitions inside the repeated portion.
//!
//! # Part two
//!
//! Same as part one but recursively expanded nested repeated sections.
//!
//! # Algorithm
//!
//! For part one we simply count letters directly, or count `len × num` for repeated sections,
//! skipping the appropiate amount of data from the stream.
//!
//! For part two, we keep a stack of repeated sections, along with the number of repetitions at
//! the current level, and the position in the stream where each repeated section ends.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Explosives in Cyberspace";
	year = 2016;
	day = 9;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
