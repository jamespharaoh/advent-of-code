//! Advent of Code 2016: Day 4: Security Through Obscurity
//!
//! [https://adventofcode.com/2016/day/4](https://adventofcode.com/2016/day/4)
//!
//! # Input
//!
//! A series of "encrypted" room names, each on its own line. A room name consists of one or more
//! sequences of lowercase ASCII letters separated by a single hyphen, followed by a hyphen and a
//! positive integer sector number, and finally a checksum in square brackets consisting of five
//! lowercase ASCII letters.
//!
//! # Part one
//!
//! Verify the checksums of each room and add up the sector numbers of the ones which are valid.
//! The checksum is valid if it consists of the five most common letters in the name, in order.
//!
//! # Part two
//!
//! Find the room name which decrypts to "northpole object storage". To decode a room name, rotate
//! each letter by the sector number, and replace hypen with space.
//!
//! # Algorithm
//!
//! This is fairly straightforward, just a little tricky. We have a function to verify if the
//! checksum is valid which we use to filter the entries, and another which is used in part two to
//! decode it and find the right room. With both of these it's fairly simple to construct a
//! chain of iterator adaptors.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Security Through Obscurity";
	year = 2016;
	day = 4;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
