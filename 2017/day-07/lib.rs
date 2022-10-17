//! Advent of Code 2017: Day 7: Recursive Circus
//!
//! [https://adventofcode.com/2017/day/7](https://adventofcode.com/2017/day/7)
//!
//! # Input
//!
//! A series of "programs", one per line, each with a name, a weight in brackets, and an optional
//! list of "held" programs. For example:
//!
//! ```text
//! prog_0 (123)
//! prog_1 (123) -> prog_2, prog_3
//! ```
//!
//! # Part one
//!
//! The programs represent a tree, so there is one program which is not "held" by another other,
//! and which holds all others. Return its name.
//!
//! # Part two
//!
//! The programs should be balanced, with the total weight of every child tree being equal for
//! every node. The provided program is not balanced, but it can be made so by modifying the
//! weight of a single program. Work out the program and return the weight it should have.
//!
//! # Algorithm
//!
//! To identify the root node, we work out the set of programs which are the children of another,
//! then iterate over all the program names looking for the single one which does not appear in
//! this set. We can then use this to build a recursive data structure with a single root where
//! each node contains a list of its children.
//!
//! For part two, we descend the tree, starting from the root, then iteratively descending to the
//! child which needs to be adjusted. We also track the weight which we expect. Once we find a
//! node which has balanced children itself, we adjust its weight accordingly, and return the
//! result.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Recursive Circus";
	year = 2017;
	day = 7;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
