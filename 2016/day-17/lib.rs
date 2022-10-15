//! Advent of Code 2016: Day 17: Two Steps Forward
//!
//! [https://adventofcode.com/2016/day/17](https://adventofcode.com/2016/day/17)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_md5 as md5;

pub mod input;
pub mod logic;

puzzle_info! {
	name = "Two Steps Forward";
	year = 2016;
	day = 17;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {
	pub type Dir = aoc_pos::Dir2d;
	pub type Pos = aoc_pos::PosYX <u8>;
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLES: & [& str] = & [
		"ihgpwlah",
		"kglvqrro",
		"ulqzkmiv",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("DDRRRD", puzzle.part_one (& [ EXAMPLES [0] ]));
		assert_eq_ok! ("DDUDRLRRUDRD", puzzle.part_one (& [ EXAMPLES [1] ]));
		assert_eq_ok! ("DRURDRUDDLLDLUURRDULRLDUUDDDRR", puzzle.part_one (& [ EXAMPLES [2] ]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("370", puzzle.part_two (& [ EXAMPLES [0] ]));
		assert_eq_ok! ("492", puzzle.part_two (& [ EXAMPLES [1] ]));
		assert_eq_ok! ("830", puzzle.part_two (& [ EXAMPLES [2] ]));
	}

}
