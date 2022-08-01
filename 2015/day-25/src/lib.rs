//! Advent of Code 2015: Day 25: Let It Snow
//!
//! [https://adventofcode.com/2015/day/25](https://adventofcode.com/2015/day/25)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Let It Snow";
	year = 2015;
	day = 25;
	parse = |input| model::Input::parse (input [0]);
	part_one = |input| logic::part_one (& input);
}

/// Logic for solving the puzzles.
///
pub mod logic {

	use super::*;
	use model::Input;
	use nums::Int;

	pub fn part_one (input: & Input) -> GenResult <u64> {
		let diag_num = u64::add_2 (input.row, input.col) ?;
		let diag_seq = u64::mul_2 (diag_num, u64::add_2 (diag_num, 1) ?) ? / 2;
		let mut cell_seq = u64::add_2 (diag_seq, input.col) ?;
		let mut code: u64 = 20_151_125;
		let mut mul = 252_533;
		while cell_seq != 0 {
			if cell_seq & 1 == 1 { code = code * mul % 33_554_393; }
			cell_seq >>= 1_i32;
			mul = mul * mul % 33_554_393;
		}
		Ok (code)
	}

}

/// Representation of the puzzle input, etc.
///
pub mod model {

	use super::*;
	use parser::*;

	#[ derive (Clone, Debug) ]
	pub struct Input {
		pub row: u64,
		pub col: u64,
	}

	impl Input {
		pub fn parse (input: & str) -> GenResult <Self> {
			Parser::wrap (input, |parser| {
				let row: u64 = parser
					.expect ("To continue, please consult the code grid in the manual.  ") ?
					.expect ("Enter the code at row ") ?
					.int () ?;
				let col: u64 = parser.expect (", column ") ?.int () ?;
				parser.expect (".") ?.end () ?;
				if row < 1 || col < 1 { Err ("Row and column start at one") ?; }
				Ok (Self { row: row - 1, col: col - 1 })
			}).map_parse_err (|col_idx| format! ("Invalid input: col {}: {}", col_idx + 1, input))
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		macro_rules! ex {
			($row:literal, $col_1:literal, $col_2:literal, $col_3:literal, $col_4:literal,
					$col_5:literal, $col_6:literal) => {
				ex! (@one, $row, "column 1", $col_1); ex! (@one, $row, "column 2", $col_2);
				ex! (@one, $row, "column 3", $col_3); ex! (@one, $row, "column 4", $col_4);
				ex! (@one, $row, "column 5", $col_5); ex! (@one, $row, "column 6", $col_6);
			};
			(@one, $row:literal, $col:literal, $expect:literal) => {
				assert_eq_ok! ($expect, puzzle.part_one (& [ & format! (
					"To continue, please consult the code grid in the manual.  Enter the code at \
					{}, {}.", $row, $col) ]));
			};
		}
		ex! ("row 1", "20151125", "18749137", "17289845", "30943339", "10071777", "33511524");
		ex! ("row 2", "31916031", "21629792", "16929656", "7726640", "15514188", "4041754");
		ex! ("row 3", "16080970", "8057251", "1601130", "7981243", "11661866", "16474243");
		ex! ("row 4", "24592653", "32451966", "21345942", "9380097", "10600672", "31527494");
		ex! ("row 5", "77061", "17552253", "28094349", "6899651", "9250759", "31663883");
		ex! ("row 6", "33071741", "6796745", "25397450", "24659492", "1534922", "27995004");
	}

}

