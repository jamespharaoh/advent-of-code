//! Advent of Code 2021: Day 4: Giant Squid
//!
//! [https://adventofcode.com/2021/day/4](https://adventofcode.com/2021/day/4)
//!
//! # Input
//!
//! TODO
//!
//! # Part one
//!
//! Work out which board will win first. Add up all numbers which haven't been called and multiply
//! by the last number called.
//!
//! # Part two
//!
//! Same as part two, but for the last board to win.
//!
//! # Algorithm
//!
//! For each board we work which turn it will win, find the lowest/highest, and then work out the
//! score.
//!
//! To work out the winning turn, we create a new board replacing the numbers with the turn on
//! which they are called. We then find the highest turn from each line, and then the lowest of
//! these is the turn on which the board will win.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod model;
pub mod logic;

puzzle_info! {
	name = "Giant Squid";
	year = 2021;
	day = 4;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
