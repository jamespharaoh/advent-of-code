//! Advent of Code 2017: Day 14: Disk Defragmentation
//!
//! [https://adventofcode.com/2017/day/14](https://adventofcode.com/2017/day/14)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_2017_knot as knot;
use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Disk Defragmentation";
	year = 2017;
	day = 14;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("8108", puzzle.part_one (& [ "flqrgnkx" ]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("1242", puzzle.part_two (& [ "flqrgnkx" ]));
	}

}
