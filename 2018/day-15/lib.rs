//! Advent of Code 2018: Day 15: Beverage Bandits
//!
//! [https://adventofcode.com/2018/day/15](https://adventofcode.com/2018/day/15)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_pos as pos;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Beverage Bandits";
	year = 2018;
	day = 15;
	parse = |input_lines| input::Input::parse_from_lines (input_lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_0: & [& str] = & [
		"#######",
		"#.G...#",
		"#...EG#",
		"#.#.#G#",
		"#..G#E#",
		"#.....#",
		"#######",
	];

	const EXAMPLE_1: & [& str] = & [
		"#######",
		"#G..#E#",
		"#E#E.E#",
		"#G.##.#",
		"#...#E#",
		"#...E.#",
		"#######",
	];

	const EXAMPLE_2: & [& str] = & [
		"#######",
		"#E..EG#",
		"#.#G.E#",
		"#E.##E#",
		"#G..#.#",
		"#..E#.#",
		"#######",
	];

	const EXAMPLE_3: & [& str] = & [
		"#######",
		"#E.G#.#",
		"#.#G..#",
		"#G.#.G#",
		"#G..#.#",
		"#...E.#",
		"#######",
	];

	const EXAMPLE_4: & [& str] = & [
		"#######",
		"#.E...#",
		"#.#..G#",
		"#.###.#",
		"#E#G#G#",
		"#...#G#",
		"#######",
	];

	const EXAMPLE_5: & [& str] = & [
		"#########",
		"#G......#",
		"#.E.#...#",
		"#..##..G#",
		"#...##..#",
		"#...#...#",
		"#.G...G.#",
		"#.....G.#",
		"#########",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("27730", puzzle.part_one (EXAMPLE_0));
		assert_eq_ok! ("36334", puzzle.part_one (EXAMPLE_1));
		assert_eq_ok! ("39514", puzzle.part_one (EXAMPLE_2));
		assert_eq_ok! ("27755", puzzle.part_one (EXAMPLE_3));
		assert_eq_ok! ("28944", puzzle.part_one (EXAMPLE_4));
		assert_eq_ok! ("18740", puzzle.part_one (EXAMPLE_5));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("4988", puzzle.part_two (EXAMPLE_0));
		assert_eq_ok! ("29064", puzzle.part_two (EXAMPLE_1));
		assert_eq_ok! ("31284", puzzle.part_two (EXAMPLE_2));
		assert_eq_ok! ("3478", puzzle.part_two (EXAMPLE_3));
		assert_eq_ok! ("6474", puzzle.part_two (EXAMPLE_4));
		assert_eq_ok! ("1140", puzzle.part_two (EXAMPLE_5));
	}

}
