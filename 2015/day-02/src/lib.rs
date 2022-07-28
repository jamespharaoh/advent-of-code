//! Advent of Code 2015: Day 2: I Was Told There Would Be No Math
//!
//! [https://adventofcode.com/2015/day/2](https://adventofcode.com/2015/day/2)

use aoc_common::*;

puzzle_info! {
	name = "I Was Told There Would Be No Math";
	year = 2015;
	day = 2;
	parse = |input| model::parse_input (input);
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

pub mod logic {

	use super::*;
	use model::Dim;
	use model::Input;
	use nums::Int;

	pub fn part_one (input: Input) -> GenResult <Dim> {
		Ok (
			input.iter_vals ().map (|(l, w, h)|
				Int::add_4 (
					Int::mul_3 (2, l, w) ?,
					Int::mul_3 (2, w, h) ?,
					Int::mul_3 (2, h, l) ?,
					[ Int::mul_2 (l, w) ?, Int::mul_2 (w, h) ?, Int::mul_2 (h, l) ?].iter_vals ()
						.min ().unwrap (),
				)
			).fold (Ok (0), |sum, val| Int::add_2 (sum ?, val ?)) ?
		)
	}

	pub fn part_two (input: Input) -> GenResult <Dim> {
		Ok (
			input.iter_vals ().map (|(l, w, h)|
				Int::add_2 (
					[
						Int::mul_2 (2, Int::add_2 (l, w) ?) ?,
						Int::mul_2 (2, Int::add_2 (w, h) ?) ?,
						Int::mul_2 (2, Int::add_2 (h, l) ?) ?,
					].iter_vals ().min ().unwrap (),
					Int::mul_3 (l, w, h) ?,
				)
			).fold (Ok (0), |sum, val| Int::add_2 (sum ?, val ?)) ?
		)
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn part_one () {
			assert_eq_ok! (0, logic::part_one (vec! []));
			assert_eq_ok! (101, logic::part_one (vec! [(2, 3, 4), (1, 1, 10)]));
			const BIG: u32 = 24770;
			assert_is_ok! (logic::part_one (vec! [(BIG, BIG, BIG)]));
			assert_err! ("Overflow", logic::part_one (vec! [(BIG + 1, BIG + 1, BIG + 1)]));
			assert_err! ("Overflow", logic::part_one (vec! [(BIG, BIG, BIG), (BIG, BIG, BIG)]));
		}

		#[ test ]
		fn part_two () {
			assert_eq_ok! (0, logic::part_two (vec! []));
			assert_eq_ok! (48, logic::part_two (vec! [(2, 3, 4), (1, 1, 10)]));
			const BIG: u32 = 1625;
			assert_is_ok! (logic::part_two (vec! [(BIG, BIG, BIG)]));
			assert_err! ("Overflow", logic::part_two (vec! [(BIG + 1, BIG + 1, BIG + 1)]));
			assert_err! ("Overflow", logic::part_two (vec! [(BIG, BIG, BIG), (BIG, BIG, BIG)]));
		}

	}

}

pub mod model {

	use super::*;

	pub type Dim = u32;
	pub type Input = Vec <(Dim, Dim, Dim)>;

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		use parser::*;
		input.iter ().enumerate ().map (|(line_idx, line)|
			Parser::wrap (line, |parser|
				Ok ((
					parser.int () ?,
					parser.expect ("x") ?.int () ?,
					parser.expect ("x") ?.int () ?,
				))
			).map_parse_err (|char_idx|
				format! ("Invalid input: line {}: col {}: {}",
					line_idx + 1, char_idx + 1, line)
			)
		).collect::<Result::<_, _>> ()
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn parse_input () -> GenResult <()> {
			assert_eq! (vec! [(1, 2, 3), (4, 5, 6)], model::parse_input (& ["1x2x3", "4x5x6"]) ?);
			assert_err! ("Invalid input: line 2: col 2: 4xx5x6",
				model::parse_input (& ["1x2x3", "4xx5x6"]));
			Ok (())
		}

	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [ "2x3x4", "1x1x10" ];

	#[ test ]
	fn part_one () -> GenResult <()> {
		let puzzle = puzzle_metadata ();
		assert_eq! ("58", puzzle.part_one (& EXAMPLE [0 .. 1]) ?);
		assert_eq! ("43", puzzle.part_one (& EXAMPLE [1 .. 2]) ?);
		assert_eq! ("101", puzzle.part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		let puzzle = puzzle_metadata ();
		assert_eq! ("34", puzzle.part_two (& EXAMPLE [0 .. 1]) ?);
		assert_eq! ("14", puzzle.part_two (& EXAMPLE [1 .. 2]) ?);
		assert_eq! ("48", puzzle.part_two (EXAMPLE) ?);
		Ok (())
	}

}
