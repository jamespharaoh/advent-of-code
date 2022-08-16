//! Advent of Code 2016: Day 9: Explosives in Cyberspace
//!
//! [https://adventofcode.com/2016/day/9](https://adventofcode.com/2016/day/9)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Explosives in Cyberspace";
	year = 2016;
	day = 9;
	part_one = |input| logic::part_one (input [0]);
	part_two = |input| logic::part_two (input [0]);
}

pub mod logic {

	use super::*;

	pub fn part_one (input: & str) -> GenResult <usize> {
		let sum = Parser::wrap (input, |parser| {
			let mut sum = 0_usize;
			while parser.peek ().is_some () {
				if parser.peek ().unwrap () == '(' {
					let len: usize = parser.expect ("(") ?.int () ?;
					let num: usize = parser.expect ("x") ?.int () ?;
					parser.expect (")") ?;
					for _ in 0 .. len {
						parser.expect_next () ?;
						sum = usize::add_2 (sum, num) ?;
					}
				} else {
					parser.expect_next () ?;
					sum += 1;
				}
			}
			Ok (sum)
		}).map_parse_err (|_, col_idx|
			format! ("Invalid input: col {}: {}", col_idx + 1, input)) ?;
		Ok (sum)
	}

	pub fn part_two (input: & str) -> GenResult <usize> {
		let sum = Parser::wrap (input, |parser| {
			let mut sum = 0;
			while let Some (ch) = parser.next () {
				if ch == '(' {
					let len: usize = parser.int () ?;
					let num: usize = parser.expect ("x") ?.int () ?;
					parser.expect (")") ?;
					let byte_len = parser.peek_rest ().chars ()
						.take (len)
						.map (char::len_utf8)
						.sum ();
					sum = usize::add_2 (sum,
						usize::mul_2 (num,
							part_two (
								#[ allow (clippy::string_slice) ]
								& parser.peek_rest () [0 .. byte_len],
							) ?,
						) ?,
					) ?;
					for _ in 0 .. len { parser.next ().ok_or_else (|| parser.err ()) ?; }
				} else {
					sum += 1;
				}
			}
			Ok (sum)
		}).map_parse_err (|_, col_idx|
			format! ("Invalid input: col {}: {}", col_idx + 1, input)) ?;
		Ok (sum)
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLES_ONE: & [& str] = & [
		"ADVENT",
		"A(1x5)BC",
		"(3x3)XYZ",
		"A(2x2)BCD(2x2)EFG",
		"(6x1)(1x3)A",
		"X(8x2)(3x3)ABCY",
	];

	const EXAMPLES_TWO: & [& str] = & [
		"(3x3)XYZ",
		"X(8x2)(3x3)ABCY",
		"(27x12)(20x12)(13x14)(7x10)(1x12)A",
		"(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("6", puzzle.part_one (& [& EXAMPLES_ONE [0]]));
		assert_eq_ok! ("7", puzzle.part_one (& [& EXAMPLES_ONE [1]]));
		assert_eq_ok! ("9", puzzle.part_one (& [& EXAMPLES_ONE [2]]));
		assert_eq_ok! ("11", puzzle.part_one (& [& EXAMPLES_ONE [3]]));
		assert_eq_ok! ("6", puzzle.part_one (& [& EXAMPLES_ONE [4]]));
		assert_eq_ok! ("18", puzzle.part_one (& [& EXAMPLES_ONE [5]]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("9", puzzle.part_two (& [& EXAMPLES_TWO [0]]));
		assert_eq_ok! ("20", puzzle.part_two (& [& EXAMPLES_TWO [1]]));
		assert_eq_ok! ("241920", puzzle.part_two (& [& EXAMPLES_TWO [2]]));
		assert_eq_ok! ("445", puzzle.part_two (& [& EXAMPLES_TWO [3]]));
	}

}
