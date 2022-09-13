//! Advent of Code 2019: Day 19: Tractor Beam
//!
//! [https://adventofcode.com/2019/day/19](https://adventofcode.com/2019/day/19)
//!
//! # Input
//!
//! Comma separated list of initial memory values for [`intcode`] program.
//!
//! # Part one
//!
//! Run the programme with two inputs representing `x` and `y` coordinates, then read a single
//! output which is `1` or `0`. Count the number of points with coordinates in the range
//! `(0 .. 50)` for which the output is `1`.
//!
//! # Part two
//!
//! Find the closest square of `100x100` to the origin of points which all generate an output of
//! `1`. Return the `x` coordinate times `10,000` plus the `y` coordinate.
//!
//! # Algorithm
//!
//! The points which return `1` are contained within two straight lines stretching out from the
//! origin. Only positive coordinate values need to be considered. This allows us to optimise in
//! a few ways.
//!
//! For part one, we consider a square of increasing size starting at the origin. Every time the
//! square grows by one, we consider the new points added. Assuming any points are active, we only
//! need to find the range of points which are active. We also know the number of inactive points
//! at each end can only increase. We iterate from both ends, skipping the number of inactive
//! points from the previous iteration, and look for the first active point. With this information
//! we can deduce the total number of active points in this square which weren't included in the
//! previous one.
//!
//! For part two we start off with something similar, searching ever increasing squares, but we
//! are looking for any `100x100` square which may contain an origin. If the bottom right point in
//! a square is not active, then none of the 10,000 points in the square could be a valid origin,
//! since every one of them would have to include this point.
//!
//! Once we find a square which may contain an origin, we repeat the same process with increased
//! granularity until we find some matches. Since we are searching a square size which doesn't
//! correspond to the final square size, we start checking four points instead of a single one.
//! These are still aligned to the bottom right, eg to check if a square of `25x25` might be valid
//! at the origin then we check `24,24`, `24,99`, `99,24` and `99,99`.
//!
//! Every time we increase the search radius for the initial `100x100` square, we find all possible
//! results before returning and then pick the largest one. We also always cache results because
//! running the actual programme is rather slow.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_2019_intcode as intcode;
use aoc_common::*;
use aoc_pos as pos;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Tractor Beam";
	year = 2019;
	day = 19;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
