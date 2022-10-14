//! Advent of Code 2016: Day 1: No Time for a Taxicab
//!
//! [https://adventofcode.com/2016/day/1](https://adventofcode.com/2016/day/1)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "No Time for a Taxicab";
	year = 2016;
	day = 1;
	parse = |input| model::Input::parse (input [0]);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Dir;
	use model::Input;
	use model::Pos;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		let mut dir = Dir::North;
		let mut pos = Pos::ZERO;
		for & (turn, dist) in & input.steps {
			dir = dir + turn;
			pos = (pos + (dir, dist)) ?;
		}
		let dist = pos.n.unsigned_abs () + pos.e.unsigned_abs ();
		Ok (dist)
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		let mut dir = Dir::North;
		let mut pos = Pos::ZERO;
		let mut seen = HashSet::new ();
		let mut twice = None;
		seen.insert (pos);
		'OUTER: for & (turn, dist) in & input.steps {
			dir = dir + turn;
			for _ in 0_i32 .. dist {
				pos = (pos + (dir, 1_i32)) ?;
				if ! seen.insert (pos) { twice = Some (pos); break 'OUTER }
			}
		}
		let dist = if let Some (twice) = twice {
			twice.n.unsigned_abs () + twice.e.unsigned_abs ()
		} else { Err ("No solution found") ? };
		Ok (dist)
	}

}

pub mod model {

	use super::*;

	pub type Dir = aoc_pos::DirGeo;
	pub type Pos = aoc_pos::PosGeo <i32>;
	pub type Turn = aoc_pos::Turn2d;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub steps: Vec <(Turn, i32)>,
	}

	impl Input {
		pub fn parse (input: & str) -> GenResult <Self> {
			Parser::wrap (input, |parser| {
				let mut steps = Vec::new ();
				loop {
					let dir = match parser.expect_next () ? {
						'L' => Turn::Left,
						'R' => Turn::Right,
						_ => Err (parser.err ()) ?,
					};
					let dist = parser.int () ?;
					if dist < 1_i32 { Err ("Min distance is 1") ?; }
					if dist > 1000_i32 { Err ("Max distance is 1000") ?; }
					steps.push ((dir, dist));
					if parser.peek ().is_none () { break }
					parser.expect (", ") ?;
				}
				Ok (Self { steps })
			}).map_parse_err (|_, col_idx|
				format! ("Invalid input: col {}: {}", col_idx + 1, input))
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLES: & [& str] = & [
		"R2, L3",
		"R2, R2, R2",
		"R5, L5, R5, R3",
		"R8, R4, R4, R8",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("5", puzzle.part_one (& [EXAMPLES [0]]));
		assert_eq_ok! ("2", puzzle.part_one (& [EXAMPLES [1]]));
		assert_eq_ok! ("12", puzzle.part_one (& [EXAMPLES [2]]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("4", puzzle.part_two (& [EXAMPLES [3]]));
	}

}
