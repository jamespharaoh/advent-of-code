//! Advent of Code 2018: Day 21: Chronal Conversion
//!
//! [https://adventofcode.com/2018/day/21](https://adventofcode.com/2018/day/21)
//!
//! # Input
//!
//! A single line defining the instruction pointer register eg "#ip 0", then a series of lines with
//! an opcode name followed by three integer arguments.
//!
//! # Part one
//!
//! Work out the starting value for register 0 which makes the program exit as quickly as possible.
//!
//! # Part two
//!
//! Work out the starting value for register 0 which makes the program exit as slowly as possible.
//!
//! # Algorithm
//!
//! Run the programme as normal but look out for any instructions which look at register 0. The
//! only supported opcode is "eqrr" and we split our iteration in two, with one branch assuming
//! equality and the other ineqaulity, taking the equal branch first. The value reg 0 was compared
//! about is also stored and this is taken when the programme halts.
//!
//! There are quite a few shortcuts taken here, but I think it will work for any provided puzzle
//! input from the official site.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_2018_cpu as cpu;

pub mod analyser;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Chronal Conversion";
	year = 2018;
	day = 21;
	parse = |input_lines| input::Input::parse_from_lines (input_lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "analyse"; method = analyser::run; ),
	];
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"#ip 5",
		"seti 9 0 1",
		"addi 1 1 1", // incr
		"eqri 1 20 2",
		"addr 5 2 5",
		"addi 5 2 5", // goto check
		"seti 9 0 1",
		"seti 0 0 5", // goto incr
		"eqrr 1 0 2", // check
		"addr 5 2 5",
		"seti 0 0 5", // goto incr
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("10", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("19", puzzle.part_two (EXAMPLE));
	}

}
