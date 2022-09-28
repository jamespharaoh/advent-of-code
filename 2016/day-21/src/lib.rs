//! Advent of Code 2016: Day 21: Scrambled Letters and Hash
//!
//! [https://adventofcode.com/2016/day/21](https://adventofcode.com/2016/day/21)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Scrambled Letters and Hash";
	year = 2016;
	day = 21;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;
	use model::ScrambleOp;

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	enum Mode { Scramble, Unscramble }

	pub fn part_one (input: & Input) -> GenResult <String> {
		calc_result (& input.ops, & input.start_one, Mode::Scramble)
	}

	pub fn part_two (input: & Input) -> GenResult <String> {
		calc_result (& input.ops, & input.start_two, Mode::Unscramble)
	}

	fn calc_result (ops: & [ScrambleOp], start: & str, mode: Mode) -> GenResult <String> {

		use Mode::{ Scramble, Unscramble };
		use ScrambleOp::{ Move, Reverse, RotChar, RotLeft, RotRight, SwapChars, SwapPosns };

		// check password length

		if start.is_empty () {
			return Err ("Password must have at least one character".into ());
		}
		if mode == Unscramble && start.chars ().count () != 8 {
			return Err ("Can only unscramble password with length 8".into ());
		}

		// convert password to chars, allocate temp buffer

		let mut state: Vec <char> = start.chars ().collect ();
		let mut state_temp: Vec <char> = Vec::with_capacity (state.len ());

		// iterate over operations, in reverse order for unscramble

		let ops_iter = match mode {
			Scramble => Either::Left (ops.iter ().copied ()),
			Unscramble => Either::Right (ops.iter ().rev ().copied ()),
		};
		for op in ops_iter {
			match (mode, op) {
				(_, SwapPosns (pos_0, pos_1)) => {
					let pos_0 = pos_0.pan_usize ();
					let pos_1 = pos_1.pan_usize ();
					if pos_0 >= state.len () || pos_1 >= state.len () {
						return Err ("Swap positions must be inside password".into ());
					}
					let ch_0 = state [pos_0];
					let ch_1 = state [pos_1];
					state [pos_0] = ch_1;
					state [pos_1] = ch_0;
				},
				(_, SwapChars (ch_0, ch_1)) => {
					for ch in state.iter_mut () {
						* ch = match * ch {
							ch if ch == ch_0 => ch_1,
							ch if ch == ch_1 => ch_0,
							ch => ch,
						}
					}
				},
				(Scramble, RotLeft (num)) | (Unscramble, RotRight (num)) => {
					let num = num.pan_usize () % state.len ();
					state_temp.clear ();
					state_temp.extend (state.iter_vals ().skip (num));
					state_temp.extend (state.iter_vals ().take (num));
					mem::swap (& mut state, & mut state_temp);
				},
				(Scramble, RotRight (num)) | (Unscramble, RotLeft (num)) => {
					let num = num.pan_usize () % state.len ();
					state_temp.clear ();
					state_temp.extend (state.iter_vals ().skip (state.len () - num));
					state_temp.extend (state.iter_vals ().take (state.len () - num));
					mem::swap (& mut state, & mut state_temp);
				},
				(Scramble, RotChar (ch)) => {
					if let Some (pos) = state.iter_vals ().position (|some_ch| some_ch == ch) {
						let num = (if pos >= 4 { 2 } else { 1 } + pos) % state.len ();
						state_temp.clear ();
						state_temp.extend (state.iter_vals ().skip (state.len () - num));
						state_temp.extend (state.iter_vals ().take (state.len () - num));
						mem::swap (& mut state, & mut state_temp);
					}
				},
				(Unscramble, RotChar (ch)) => {
					if let Some (pos) = state.iter_vals ().position (|some_ch| some_ch == ch) {
						let num = match pos {
							0 => 1, 1 => 1, 2 => 6, 3 => 2,
							4 => 7, 5 => 3, 6 => 0, 7 => 4,
							_ => unreachable! (),
						};
						state_temp.clear ();
						state_temp.extend (state.iter_vals ().skip (num));
						state_temp.extend (state.iter_vals ().take (num));
						mem::swap (& mut state, & mut state_temp);
					}
				},
				(_, Reverse (pos_0, pos_1)) => {
					let mut pos_0 = pos_0.pan_usize ();
					let mut pos_1 = pos_1.pan_usize ();
					if pos_0 >= state.len () || pos_1 >= state.len () {
						return Err ("Reverse range outside of password length".into ());
					}
					while pos_0 < pos_1 {
						let ch_0 = state [pos_0];
						let ch_1 = state [pos_1];
						state [pos_0] = ch_1;
						state [pos_1] = ch_0;
						pos_0 += 1;
						pos_1 -= 1;
					}
				},
				(Scramble, Move (pos_0, pos_1)) | (Unscramble, Move (pos_1, pos_0)) => {
					let pos_0 = pos_0.pan_usize ();
					let pos_1 = pos_1.pan_usize ();
					if pos_0 >= state.len () || pos_1 >= state.len () {
						return Err ("Move position outside of password length".into ());
					}
					let ch = state.remove (pos_0);
					state.insert (pos_1, ch);
				},
			}
		}

		// collect back into string and return

		let result = state.into_iter ().collect::<String> ();
		Ok (result)

	}

}

pub mod model {

	use super::*;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub ops: Vec <ScrambleOp>,
		pub start_one: String,
		pub start_two: String,
	}

	#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
	pub enum ScrambleOp {
		SwapPosns (u32, u32),
		SwapChars (char, char),
		RotLeft (u32),
		RotRight (u32),
		RotChar (char),
		Reverse (u32, u32),
		Move (u32, u32),
	}

	impl Input {

		pub fn parse (mut input: & [& str]) -> GenResult <Self> {
			let start_one = parser::input_param (& mut input, "START_ONE=", "abcdefgh") ?;
			let start_two = parser::input_param (& mut input, "START_TWO=", "fbgdceah") ?;
			let ops: Vec <_> = input.iter ().enumerate ()
				.map (|(line_idx, line)| {
					#[ allow (clippy::redundant_closure_for_method_calls) ]
					Parser::wrap (line, |parser| parser.item ())
						.map_parse_err (|_, col_idx|
							format! ("Invalid input: line {}: col {}: {}",
								line_idx + 1, col_idx + 1, line))
				})
				.collect::<GenResult <_>> () ?;
			if ops.is_empty () { return Err ("Must have at least one scramble op".into ()) }
			Ok (Self { ops, start_one, start_two })
		}

	}

	impl <'inp> FromParser <'inp> for ScrambleOp {
		fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
			parser.any ()
				.of (|parser| {
					let pos_0 = parser.expect ("swap position ") ?.confirm ().int () ?;
					let pos_1 = parser.expect (" with position ") ?.int () ?;
					parser.end () ?;
					Ok (Self::SwapPosns (pos_0, pos_1))
				})
				.of (|parser| {
					let ch_0 = parser.expect ("swap letter ") ?.confirm ().expect_next () ?;
					let ch_1 = parser.expect (" with letter ") ?.expect_next () ?;
					parser.end () ?;
					Ok (Self::SwapChars (ch_0, ch_1))
				})
				.of (|parser| {
					let num = parser.expect ("rotate left ") ?.confirm ().int () ?;
					parser.expect (if num == 1 { " step" } else { " steps" }) ?.end () ?;
					Ok (Self::RotLeft (num))
				})
				.of (|parser| {
					let num = parser.expect ("rotate right ") ?.confirm ().int () ?;
					parser.expect (if num == 1 { " step" } else { " steps" }) ?.end () ?;
					Ok (Self::RotRight (num))
				})
				.of (|parser| {
					let ch = parser.expect ("rotate based on position of letter ") ?.confirm ()
						.expect_next () ?;
					parser.end () ?;
					Ok (Self::RotChar (ch))
				})
				.of (|parser| {
					let pos_0 = parser.expect ("reverse positions ") ?.confirm ().int () ?;
					let pos_1 = parser.expect (" through ") ?.int () ?;
					parser.end () ?;
					Ok (Self::Reverse (pos_0, pos_1))
				})
				.of (|parser| {
					let pos_0 = parser.expect ("move position ") ?.confirm ().int () ?;
					let pos_1 = parser.expect (" to position ") ?.int () ?;
					parser.end () ?;
					Ok (Self::Move (pos_0, pos_1))
				})
				.done ()
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"START_ONE=abcde",
		"START_TWO=fbdecgha",
		"swap position 4 with position 0",
		"swap letter d with letter b",
		"reverse positions 0 through 4",
		"rotate left 1 step",
		"move position 1 to position 4",
		"move position 3 to position 0",
		"rotate based on position of letter b",
		"rotate based on position of letter d",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("decab", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("abcdefgh", puzzle.part_two (EXAMPLE));
	}

}
