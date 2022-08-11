//! Advent of Code 2017: Day 2: Corruption Checksum
//!
//! [https://adventofcode.com/2017/day/2](https://adventofcode.com/2017/day/2)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Corruption Checksum";
	year = 2017;
	day = 2;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		Ok (
			input.rows.iter ()
				.map (|row| {
					let max = row.iter_vals ().max ().unwrap_or (0).as_u32 ();
					let min = row.iter_vals ().min ().unwrap_or (0).as_u32 ();
					max - min
				})
				.sum ()
		)
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		input.rows.iter ()
			.map (|row| row.iter_vals ()
				.enumerate ()
				.flat_map (|(idx_0, val_0)| row.iter_vals ()
					.skip (idx_0 + 1)
					.filter_map (move |val_1| {
						let val_low = cmp::min (val_0, val_1);
						let val_high = cmp::max (val_0, val_1);
						(val_high % val_low == 0).then_some (val_high / val_low)
					}))
				.exactly_one ()
				.map_err (|_err| GenError::from ("No solution found")))
			.fold (Ok (0_u32), |sum, item|
				Ok::<_, GenError> (Int::add_2 (
					sum ?,
					item ?.as_u32 ()) ?))
	}

}

pub mod model {

	use super::*;

	pub type Value = u16;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub rows: Vec <Vec <Value>>,
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Self> {
			let rows = input.iter ()
				.enumerate ()
				.map (|(line_idx, line)|
					Parser::wrap (line.trim (), |parser| {
						let mut row = Vec::new ();
						while ! parser.rest ().is_empty () {
							let value = parser.int () ?;
							if value == 0 { return Err ("Zero is not allowed".into ()) }
							row.push (value);
							parser.skip_whitespace ();
						}
						Ok (row)
					}).map_parse_err (|col_idx|
						format! ("Invalid input: line {}: col {}: {}",
							line_idx + 1, col_idx + 1, line)))
				.collect::<GenResult <_>> () ?;
			Ok (Self { rows })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_ONE: & [& str] = & [
		"5 1 9 5",
		"7 5 3",
		"2 4 6 8",
	];

	const EXAMPLE_TWO: & [& str] = & [
		"5 9 2 8",
		"9 4 7 3",
		"3 8 6 5",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("18", puzzle.part_one (EXAMPLE_ONE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("9", puzzle.part_two (EXAMPLE_TWO));
	}

}
