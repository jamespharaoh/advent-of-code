//! Advent of Code 2016: Day 5: How About a Nice Game of Chess?
//!
//! [https://adventofcode.com/2016/day/5](https://adventofcode.com/2016/day/5)
//!
//! # Input
//!
//! Any string, used as a cryptographic salt. Only a single line is used. For testing purposes, we
//! also accept a prefix line in the form `"NUM_ZEROS=n"` where n is the number of zeros to match
//! at the start of the generated hash.
//!
//! # Part one
//!
//! Construct a password by searching for integers which can be appended to the input string which
//! give an md5 hash having at least 5 zeros at the start of its hex representation. The password
//! is built by appending the 6th hex digit each time. Once eight characters are obtained, the
//! password is complete.
//!
//! # Part two
//!
//! The sixth character of the hex hash now represents the position of the character in the
//! password, and the seventh is the character itself.
//!
//! # Algorithm
//!
//! We use the md5 implementation in [`aoc_common`]. Iterate over integers to append until we find
//! a match and build up the password. To make things faster we use [`aoc_parallel::ThreadMap`] to
//! generate hashes parallely in separate threads.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_parallel::{ self as parallel, prelude::* };
use aoc_md5 as md5;
use md5::md5_hash;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "How About a Nice Game of Chess?";
	year = 2016;
	day = 5;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
