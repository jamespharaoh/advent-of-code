//! Advent of Code 2017: Day 16: Permutation Promenade
//!
//! [https://adventofcode.com/2017/day/16](https://adventofcode.com/2017/day/16)
//!
//! # Input
//!
//! A single line containing a comma-separated list of "dance steps", where `X` and `Y` are
//! positions in the range `0` to `15`, and `A` and `B` are programmes in the range `a` to `p`.
//!
//! - `sX` rotates the line left by `X`
//! - `xX/Y` exchanges positions `X` and `Y`
//! - `pA/B` swaps programmes `A` and `B`
//!
//! # Part one
//!
//! Starting with the programme line `abcdefghijklmnop`, apply the steps once.
//!
//! # Part two
//!
//! Same as part one, but repeat the process one billion times.
//!
//! # Algorithm
//!
//! We split the process in two, tracking position changes separately from programme swaps, which
//! we refer to as a transformation. These transformations can then be combined giving an efficient
//! way to execute many steps in O(n) time. To apply a transformation we first apply the position
//! changes, then the programme swaps.
//!
//! First we combine all of the steps into a single transformation. We then iterate over the bits
//! in the number of times we want to apply the steps, starting with the lowest, updating the line
//! if the bit is a one. To work out the transformation for the next bit, we combine the current
//! transformation with itself.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Permutation Promenade";
	year = 2017;
	day = 16;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
