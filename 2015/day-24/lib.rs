//! Advent of Code 2015: Day 24: It Hangs in the Balance
//!
//! [https://adventofcode.com/2015/day/24](https://adventofcode.com/2015/day/24)
//!
//! # Input
//!
//! Each line contains a decimal integer, representing the weight of a single package.
//!
//! # Part one
//!
//! Split the packages into three piles of equal weight. Optimise so that one pile has the fewest
//! possible number of packages. Further optimize so that the product of the weights of the
//! packages in the first pile is as small as possible. Produce that product as a result.
//!
//! # Part two
//!
//! The same as part one, but with four piles instead of three.
//!
//! # Algorithm
//!
//! - Iteratively build up a list of items to assign to each pile, always in order from heavier to
//!   lighter. This ensures that we find a solution with the smallest possible number of packages
//!   in the first pile before any other.
//! - Once we find a solution for all three piles, record the size and product of the first pile,
//!   and only consider solutions with a first pile which would signify an improvement.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "It Hangs in the Balance";
	year = 2015;
	day = 24;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
