//! Advent of Code 2018: Day 12: Subterranean Sustainability
//!
//! [https://adventofcode.com/2018/day/12](https://adventofcode.com/2018/day/12)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Subterranean Sustainability";
	year = 2018;
	day = 12;
	parse = |input_lines| input::Input::parse_from_lines (input_lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"CHECK_RULES=false",
		"initial state: #..#.#..##......###...###",
		"",
		"...## => #",
		"..#.. => #",
		".#... => #",
		".#.#. => #",
		".#.## => #",
		".##.. => #",
		".#### => #",
		"#.#.# => #",
		"#.### => #",
		"##.#. => #",
		"##.## => #",
		"###.. => #",
		"###.# => #",
		"####. => #",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("325", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("999999999374", puzzle.part_two (EXAMPLE));
	}

}
