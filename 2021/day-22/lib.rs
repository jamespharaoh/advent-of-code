//! Advent of Code 2021: Day 22: Reactor Reboot
//!
//! [https://adventofcode.com/2021/day/22](https://adventofcode.com/2021/day/22)
//!
//! This uses a slightly unintuitive algorithm to count up the activated locations. Instead of
//! splitting up cubes to ensure they never intersect, we simply add them all together, then keep
//! track of where they overlapped. Every time we overlap we track the overlapping section as well,
//! but we add it up as the negative. If we overlap a negative we add it again as a positive.
//!
//! To make part two quicker, we split the referenced area up into a 10×10×10 grid and calculate
//! the value for each section individually. This drastically cuts down on the number of
//! intersections we need to make.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Reactor Reboot";
	year = 2021;
	day = 22;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
