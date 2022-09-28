//! Advent of Code 2016: Day 8: Two-Factor Authentication
//!
//! [https://adventofcode.com/2016/day/8](https://adventofcode.com/2016/day/8)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid::prelude::*;
use aoc_ocr as ocr;
use aoc_pos as pos;

puzzle_info! {
	name = "Two-Factor Authentication";
	year = 2016;
	day = 8;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;
	use model::Step;
	use model::Pos;

	pub fn part_one (input: & Input) -> GenResult <usize> {
		let grid = calc_result (input);
		Ok (grid.values ().filter (|& val| val).count ())
	}

	pub fn part_two (input: & Input) -> GenResult <String> {
		let grid = calc_result (input);
		let code = ocr::read_dots (
			grid.iter ()
				.filter_map (|(pos, val)| val
					.then_some ((pos.row, pos.col)))) ?;
		Ok (code)
	}

	fn calc_result (input: & Input) -> GridBuf <Vec <bool>, Pos, 2> {
		let mut grid: GridBuf <Vec <bool>, Pos, 2> =
			GridBuf::new (Pos::ZERO, Pos::new (input.height, input.width));
		for step in input.steps.iter_vals () {
			match step {
				Step::Rect { width, height } => {
					for row in 0 .. height {
						for col in 0 .. width {
							grid.set (Pos { row, col }, true);
						}
					}
				},
				Step::RotateRow { row, dist } => {
					let temp: Vec <bool> =
						(0 .. input.width)
							.map (|col| grid.get (Pos { row, col }).unwrap ())
							.collect ();
					for old_col in 0 .. input.width {
						let col = (old_col + dist) % input.width;
						grid.set (Pos { row, col }, temp [old_col.pan_usize ()]);
					}
				},
				Step::RotateCol { col, dist } => {
					let temp: Vec <bool> =
						(0 .. input.height)
							.map (|row| grid.get (Pos { row, col }).unwrap ())
							.collect ();
					for old_row in 0 .. input.height {
						let row = (old_row + dist) % input.height;
						grid.set (Pos { row, col }, temp [old_row.pan_usize ()]);
					}
				},
			}
		}
		grid
	}

}

pub mod model {

	use super::*;

	pub type Pos = pos::PosRowCol <u32>;

	#[ derive (Clone, Debug) ]
	pub struct Input {
		pub steps: Vec <Step>,
		pub width: u32,
		pub height: u32,
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Step {
		Rect { width: u32, height: u32 },
		RotateRow { row: u32, dist: u32 },
		RotateCol { col: u32, dist: u32 },
	}

	impl Input {
		pub fn parse (mut lines: & [& str]) -> GenResult <Self> {
			let width = parser::input_param (& mut lines, "WIDTH=", 50_u32) ?;
			let height = parser::input_param (& mut lines, "HEIGHT=", 6_u32) ?;
			let steps = lines.iter ()
				.enumerate ()
				.map (|(line_idx, line)|
					Parser::wrap (line, |parser| {
						parser.set_ignore_whitespace (true);
						Step::parse_real (parser)
					}).map_parse_err (|_, col_idx| format! (
						"Invalid input: line {}: col {}: {}", line_idx + 1, col_idx + 1, line)))
				.collect::<GenResult <Vec <Step>>> () ?;
			if steps.iter_vals ().any (|step|
					match step {
						Step::Rect { width: step_width, height: step_height } =>
							step_width >= width || step_height >= height,
						Step::RotateRow { row, .. } => height <= row,
						Step::RotateCol { col, .. } => width <= col,
					}) {
				Err ("Invalid input") ?;
			}
			Ok (Self { steps, width, height })
		}
	}

	impl Step {
		fn parse_real (parser: & mut Parser) -> ParseResult <Self> {
			parser.any ()
				.of (|parser| {
					let width = parser.expect ("rect") ?.confirm ().int () ?;
					let height = parser.expect ("x") ?.int () ?;
					Ok (Self::Rect { width, height })
				})
				.of (|parser| {
					let row = parser.expect ("rotate row y=") ?.confirm ().int () ?;
					let dist = parser.expect ("by") ?.int () ?;
					Ok (Self::RotateRow { row, dist })
				})
				.of (|parser| {
					let col = parser.expect ("rotate column x=") ?.confirm ().int () ?;
					let dist = parser.expect ("by") ?.int () ?;
					Ok (Self::RotateCol { col, dist })
				})
				.done ()
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"WIDTH=7",
		"HEIGHT=3",
		"rect 3x2",
		"rotate column x=1 by 1",
		"rotate row y=0 by 4",
		"rotate column x=1 by 1",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("6", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_err! ("Unrecognised character: 0x402804 << 96 in position 1", puzzle.part_two (EXAMPLE));
	}

}
