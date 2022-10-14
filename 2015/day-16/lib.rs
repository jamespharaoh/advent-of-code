//! Advent of Code 2015: Day 16: Aunt Sue
//!
//! [https://adventofcode.com/2015/day/16](https://adventofcode.com/2015/day/16)
//!
//! # Input
//!
//! A list of "aunt Sue"s, with a numbered id and information known about each one. Each piece of
//! information takes the form of the number of one of ten types of posession. The lines take the
//! following form:
//!
//! ```text
//! Sue 1: item_0: 2, item_1: 3
//! ```
//!
//! There are ten types of posessions: `children`, `cats`, `samoyeds`, `pomeranians`, `akitas`,
//! `viszlas`, `goldfish`, `trees`, `cars`, `perfumes`.
//!
//! Types of posession which are not specified could take any value.
//!
//! # Part one
//!
//! Return the id number of the sue with exactly 3 children, 7 cats, 2 samoyeds, 3 pomeranians, 0
//! akitas, 0 vizslas, 5 goldfish, 3 trees, 2 cars and 1 perfume.
//!
//! # Part two
//!
//! Return the id number of the sue with exactly 3 children, more than 7 cats, 2 samoyeds, less
//! than 3 pomeranians, 0 akitas, 0 vizslas, less than 5 goldfish, more than 3 trees, 2 cars and 1
//! perfume.
//!
//! # Algorithm
//!
//! Iterate over the sues and apply the specified. Verify that there is exactly one match and
//! return the id.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Aunt Sue";
	year = 2015;
	day = 16;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
