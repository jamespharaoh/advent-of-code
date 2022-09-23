//! Advent of Code 2018: Day 20: A Regular Map
//!
//! [https://adventofcode.com/2018/day/20](https://adventofcode.com/2018/day/20)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos::GenPosCore as _;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "A Regular Map";
	year = 2018;
	day = 20;
	parse = |input_lines| input::Input::parse_from_lines (input_lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLES: & [& str] = & [
		"^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$",
		"^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$",
		"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("18", puzzle.part_one (& [ EXAMPLES [0] ]));
		assert_eq_ok! ("23", puzzle.part_one (& [ EXAMPLES [1] ]));
		assert_eq_ok! ("31", puzzle.part_one (& [ EXAMPLES [2] ]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("13", puzzle.part_two (& [ "DIST_TWO=10", EXAMPLES [0] ]));
		assert_eq_ok! ("25", puzzle.part_two (& [ "DIST_TWO=10", EXAMPLES [1] ]));
		assert_eq_ok! ("39", puzzle.part_two (& [ "DIST_TWO=10", EXAMPLES [2] ]));
	}

}
