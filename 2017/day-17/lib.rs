//! Advent of Code 2017: Day 17: Spinlock
//!
//! [https://adventofcode.com/2017/day/17](https://adventofcode.com/2017/day/17)
//!
//! # Input
//!
//! A single line containing a positive number. This is the number of positions to advance in each
//! iteration.
//!
//! # Part one
//!
//! Begin with a circular buffer containing only the number `0`. Iterate 2017 times, moving forward
//! the number of positions specified in the input, plus one, then insert the next number after the
//! current position. The answer is the number after that in the resulting buffer.
//!
//! # Part two
//!
//! Same as part one, but iterate fifty million times, and return the number following `0` in the
//! buffer.
//!
//! # Algorithm
//!
//! For part one, we use a naïve algorithm, creating the circular buffer and inserting the number
//! each time.
//!
//! For part two, we instead only track the number that was last inserted after zero. We model the
//! circular buffer with only the following information:
//!
//! * `size`: total number of items in the buffer
//! * `pos`: current position, where `0` is at position `0`
//! * `second`: item at position `1`, ie the answer
//!
//! Instead of iterating fifty million times, we alternate between advancing towards the end of the
//! buffer quickly using division, and advancing past the end of the buffer in a single step. This
//! allows the process to complete much more quickly.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Spinlock";
	year = 2017;
	day = 17;
	parse = |input| input::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"123",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("698", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("29108944", puzzle.part_two (EXAMPLE));
	}

}
