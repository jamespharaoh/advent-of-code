//! Advent of Code 2016: Day 18: Like a Rogue
//!
//! [https://adventofcode.com/2016/day/18](https://adventofcode.com/2016/day/18)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Like a Rogue";
	year = 2016;
	day = 18;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;
	use model::Tile;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		calc_result (& input.first_row, input.num_rows_one)
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		calc_result (& input.first_row, input.num_rows_two)
	}

	fn calc_result (first_row: & [Tile], num_rows: u32) -> GenResult <u32> {
		if first_row.is_empty () { return Err ("Max row size is 1 tile".into ()) }
		if first_row.len () > 128 { return Err ("Max row size is 128 tiles".into ()) }
		let mut row: u128 = 0;
		let mut mask: u128 = 0;
		for tile in first_row.iter ().copied () {
			row <<= 1_u32;
			if tile == Tile::Trap { row |= 1; }
			mask <<= 1_u32;
			mask |= 1;
		}
		let num_tiles = mask.count_ones ();
		let mut num_safe = 0;
		for _ in 0 .. num_rows {
			num_safe += num_tiles - row.count_ones ();
			row = ((row << 1_u32) ^ (row >> 1_u32)) & mask;
		}
		Ok (num_safe)
	}

}

pub mod model {

	use super::*;
	use parser::*;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub first_row: Vec <Tile>,
		pub num_rows_one: u32,
		pub num_rows_two: u32,
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub enum Tile { Trap, Safe }

	impl Input {

		pub fn parse (mut input: & [& str]) -> GenResult <Self> {
			let num_rows_one = parser::input_param (& mut input, "NUM_ROWS_ONE=", 40) ?;
			let num_rows_two = parser::input_param (& mut input, "NUM_ROWS_TWO=", 400_000) ?;
			if input.len () != 1 { return Err ("Input must have exactly one line".into ()) }
			let first_row = Parser::wrap (input [0], |parser| {
					let mut items = Vec::new ();
					while let Some (ch) = parser.next () {
						items.push (match ch {
							'^' => Tile::Trap,
							'.' => Tile::Safe,
							_ => return Err (parser.err ()),
						});
					}
					Ok (items)
				}).map_parse_err (|col_idx|
					format! ("Invalid input: col {}: {}", col_idx + 1, input [0])) ?;
			Ok (Self { first_row, num_rows_one, num_rows_two })
		}

	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"NUM_ROWS_ONE=10",
		"NUM_ROWS_TWO=20",
		".^^.^.^^^^",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("38", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("93", puzzle.part_two (EXAMPLE));
	}

}
