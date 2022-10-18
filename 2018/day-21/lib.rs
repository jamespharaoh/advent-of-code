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
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "analyse"; method = analyser::run; ),
	];
}
