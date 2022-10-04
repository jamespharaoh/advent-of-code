//! Advent of Code 2021: Day 13: Transparent Origami
//!
//! [https://adventofcode.com/2021/day/13](https://adventofcode.com/2021/day/13)
//!
//! # Input
//!
//! First, a number of lines containing 2d points in `x,y` format, with `x` increasing to the right
//! and `y` increasing downwards. Then, after a blank line, a number of lines containing folds in
//! the form `fold along a=v` where `a` is the axis, `x` or `y`, and `v` is the point on the axis.
//!
//! # Part one
//!
//! Fold instructions merge points from after the fold to before it, in a mirror fashion. Apply the
//! first fold only, then count how many unique points remain.
//!
//! # Part two
//!
//! Apply all the folds in order, then read the result as a series of capital letters.
//!
//! # Algorithm
//!
//! This is fairly simple. We simply apply the folds as described. To read the text we use the
//! [`aoc_ocr`] library.

use aoc_common::*;
use aoc_ocr as ocr;
use aoc_pos as pos;

mod examples;
pub mod input;
pub mod logic;
pub mod model;
pub mod tool;

puzzle_info! {
	name = "Transparent Origami";
	year = 2021;
	day = 13;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "run"; method = tool::run; ),
	];
}
