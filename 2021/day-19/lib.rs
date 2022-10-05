//! Advent of Code 2021: Day 19: Beacon Scanner
//!
//! [https://adventofcode.com/2021/day/19](https://adventofcode.com/2021/day/19)
//!
//! This algorithm uses bloom filters to allow it to scale better. For each scanner we generate a
//! set of bits which have bits set according to the arrangement of the beacons they contain. If
//! there are two beacons with a specific offset between them, then a number of bits are guaranteed
//! to be set. Once we have this information for every scanner, we can prioritise the slower
//! matching process to pairs of scanners which share a large number of bits.
//!
//! Generating these hashes for each scanner is slow, so there is some further optimisation going
//! on as well. Firstly, we have to rotate scanners so that they will match. Instead of rotating
//! each scanner in every direction, we rotate scanners which we have placed in one set of
//! directions and the ones we haven't placed in another. We choose a set of directions in each
//! case to guarntee a match. Specifically we rotate placed scanners around the Z axis only, giving
//! four hashes for each scanner. We rotate unplaced scanners to move its Z axis into one of the
//! six other positions. since almost all scanners will first be unplaced and later placed, this
//! means we do a total of ten hashes for each scanner, instead of the twenty four we would have to
//! with a more naïve algorithm.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_bithash::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;
pub mod rotation;

const SCANNER_HASH_TOTAL_BITS: usize = 4096; // bits per hash (multiple of 64)
const SCANNER_HASH_ENTRY_BITS: usize = 3;    // bits per entry (pair of scanners)

puzzle_info! {
	name = "Beacon Scanner";
	year = 2021;
	day = 19;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
