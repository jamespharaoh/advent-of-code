//! Advent of Code 2016: Day 24: Air Duct Spelunking
//!
//! [https://adventofcode.com/2016/day/24](https://adventofcode.com/2016/day/24)
//!
//! # Input
//!
//! Map with `#` for wall tiles, `.` for open tiles, and digits `0` to `9` for points of interest.
//! Starting point is always `0`.
//!
//! # Part one
//!
//! Work out the shortest route starting at `0` and passing through all points of interest.
//!
//! # Part two
//!
//! Same as part one, while also returning to `0` afterwards.
//!
//! # Algorithm
//!
//! This works in two steps:
//!
//! - Work out the shortest route from every point of interest to every other. This uses a simple
//!   breadth first path-finding algorithm, starting from each of the points and recording the
//!   shortest distance to each of the others.
//!
//! - Use [`search::PrioritySearch`] to find the shortest path connecting them. As an optimisation,
//!   we sort the routes used as search nodes, except for the last item, since we don't care about
//!   the order, only the distance and the point we are setting off from for the next leg.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;
use aoc_search as search;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Air Duct Spelunking";
	year = 2016;
	day = 24;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
