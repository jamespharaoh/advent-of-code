//! Advent of Code 2017: Day 9: Stream Processing
//!
//! [https://adventofcode.com/2017/day/9](https://adventofcode.com/2017/day/9)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Stream Processing";
	year = 2017;
	day = 9;
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
		assert_eq_ok! ("1", puzzle.part_one (& [ "{}" ]));
		assert_eq_ok! ("6", puzzle.part_one (& [ "{{{}}}" ]));
		assert_eq_ok! ("5", puzzle.part_one (& [ "{{},{}}" ]));
		assert_eq_ok! ("16", puzzle.part_one (& [ "{{{},{},{{}}}}" ]));
		assert_eq_ok! ("1", puzzle.part_one (& [ "{<a>,<a>,<a>,<a>}" ]));
		assert_eq_ok! ("9", puzzle.part_one (& [ "{{<ab>},{<ab>},{<ab>},{<ab>}}" ]));
		assert_eq_ok! ("9", puzzle.part_one (& [ "{{<!!>},{<!!>},{<!!>},{<!!>}}" ]));
		assert_eq_ok! ("3", puzzle.part_one (& [ "{{<a!>},{<a!>},{<a!>},{<ab>}}" ]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("0", puzzle.part_two (& [ "<>" ]));
		assert_eq_ok! ("17", puzzle.part_two (& [ "<random characters>" ]));
		assert_eq_ok! ("3", puzzle.part_two (& [ "<<<<>" ]));
		assert_eq_ok! ("2", puzzle.part_two (& [ "<{!>}>" ]));
		assert_eq_ok! ("0", puzzle.part_two (& [ "<!!>" ]));
		assert_eq_ok! ("0", puzzle.part_two (& [ "<!!!>>" ]));
		assert_eq_ok! ("10", puzzle.part_two (& [ "<{o\"i!a,<{i<a>" ]));
	}

}
