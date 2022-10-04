//! Advent of Code 2021: Day 15: Chiton
//!
//! [https://adventofcode.com/2021/day/15](https://adventofcode.com/2021/day/15)
//!
//! # Input
//!
//! A grid of single-digit cost values, in the range `1` to `9`.
//!
//! # Part one
//!
//! Calculate the lower cost to get from the top left point to the bottom right. The cost is
//! applied on entering a square, so the top left cost is irrelevant.
//!
//! # Part two
//!
//! Same as part one, except the grid is replicated five times down and to the right, for a total
//! of twenty five times. Each time the grid is replicated in those directions, the cost of each
//! point is increased by one. A value of `9` becomes `1`.
//!
//! # Algorithm
//!
//! This is a relatively simple path finder. We use [`PrioritySearch`](search::PrioritySearch) to
//! prioritise the routes with the lowest risk, and keep a cache of the best ones so far to each
//! point so we can short-circuit appropriately.

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;
use aoc_search as search;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Chiton";
	year = 2021;
	day = 15;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
