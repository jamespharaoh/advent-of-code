//! Advent of Code 2016: Day 2: Bathroom Security
//!
//! [https://adventofcode.com/2016/day/2](https://adventofcode.com/2016/day/2)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos as pos;

puzzle_info! {
	name = "Bathroom Security";
	year = 2016;
	day = 2;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Dir;
	use model::Input;
	use model::Pos;

	pub fn part_one (input: & Input) -> GenResult <String> {
		let digits = |pos| match pos {
			Pos { row: 0, col: 0 } => Some ('1'),
			Pos { row: 0, col: 1 } => Some ('2'),
			Pos { row: 0, col: 2 } => Some ('3'),
			Pos { row: 1, col: 0 } => Some ('4'),
			Pos { row: 1, col: 1 } => Some ('5'),
			Pos { row: 1, col: 2 } => Some ('6'),
			Pos { row: 2, col: 0 } => Some ('7'),
			Pos { row: 2, col: 1 } => Some ('8'),
			Pos { row: 2, col: 2 } => Some ('9'),
			_ => None,
		};
		let code = calc_code (& input.steps, digits, Pos { row: 1, col: 1 }) ?;
		Ok (code)
	}

	pub fn part_two (input: & Input) -> GenResult <String> {
		let digits = |pos| match pos {
			Pos { row: 0, col: 2 } => Some ('1'),
			Pos { row: 1, col: 1 } => Some ('2'),
			Pos { row: 1, col: 2 } => Some ('3'),
			Pos { row: 1, col: 3 } => Some ('4'),
			Pos { row: 2, col: 0 } => Some ('5'),
			Pos { row: 2, col: 1 } => Some ('6'),
			Pos { row: 2, col: 2 } => Some ('7'),
			Pos { row: 2, col: 3 } => Some ('8'),
			Pos { row: 2, col: 4 } => Some ('9'),
			Pos { row: 3, col: 1 } => Some ('A'),
			Pos { row: 3, col: 2 } => Some ('B'),
			Pos { row: 3, col: 3 } => Some ('C'),
			Pos { row: 4, col: 2 } => Some ('D'),
			_ => None,
		};
		let code = calc_code (& input.steps, digits, Pos { row: 2, col: 0 }) ?;
		Ok (code)
	}

	fn calc_code (
		steps: & [Vec <Dir>],
		digits: fn (Pos) -> Option <char>,
		mut pos: Pos,
	) -> GenResult <String> {
		let mut code = String::new ();
		for line in steps.iter () {
			for & step in line {
				let new_pos = (pos + (step, 1)) ?;
				if digits (new_pos).is_some () { pos = new_pos; }
			}
			code.push (digits (pos).unwrap ());
		}
		Ok (code)
	}

}

pub mod model {

	use super::*;

	pub type Coord = i8;
	pub type Dir = pos::Dir2d;
	pub type Pos = pos::PosRowCol <Coord>;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub steps: Vec <Vec <Dir>>,
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Self> {
			let steps = input.iter ()
				.enumerate ()
				.map (|(line_idx, line)|
					line.chars ()
						.enumerate ()
						.map (|(col_idx, ch)| Ok (match ch {
							'U' => Dir::Up,
							'D' => Dir::Down,
							'L' => Dir::Left,
							'R' => Dir::Right,
							_ => Err (format! (
								"Invalid input: line {}: col {}: {}",
								line_idx + 1, col_idx, line)) ?,
						}))
						.collect::<GenResult <_>> ())
				.collect::<GenResult <_>> () ?;
			Ok (Self { steps })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"ULL",
		"RRDDD",
		"LURDL",
		"UUUUD",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("1985", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("5DB3", puzzle.part_two (EXAMPLE));
	}

}
