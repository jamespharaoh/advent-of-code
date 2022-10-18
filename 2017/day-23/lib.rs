//! Advent of Code 2017: Day 23: Coprocessor Conflagration
//!
//! [https://adventofcode.com/2017/day/23](https://adventofcode.com/2017/day/23)
//!
//! # Input
//!
//! Each line is a valid instruction for [`aoc_2017_cpu`].
//!
//! # Part one
//!
//! Execute the program to completion. Return the number of times a `mul` instruction was executed.
//!
//! # Part two
//!
//! Set register `a` to `1` and execute the program to completion. Return the value from register
//! `h`.
//!
//! # Algorithm
//!
//! For part one, we simply step through the program and count the `mul`s.
//!
//! Part two is much too slow to do this, so instead we optimise one part of the program. This part
//! basically checks whether the value in a register is prime, and sets another register to zero
//! if so. When we recognise this block of instructions we use our own algorithm to do the same and
//! skip past it.
//!
//! To quickly identify prime numbers, we try to divide by a list of known primes less then 1000,
//! then iterate over odd numbers over that up to the square root of the target.
//!
//! I think the output in `h` is the count of primes in some range, or perhaps not primes, but I
//! haven't bothered to properly analyse what it does apart from the block I optimised.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_2017_cpu as cpu;

mod examples;
pub mod input;
pub mod logic;

puzzle_info! {
	name = "Coprocessor Conflagration";
	year = 2017;
	day = 23;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
