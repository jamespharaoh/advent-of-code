//! Advent of Code 2017: Day 1: Inverse Captcha
//!
//! [https://adventofcode.com/2017/day/1](https://adventofcode.com/2017/day/1)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Inverse Captcha";
	year = 2017;
	day = 1;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		Ok (
			input.digits.iter ()
				.circular_tuple_windows::<(_, _)> ()
				.filter (|& (& digit_0, & digit_1)| digit_0 == digit_1)
				.map (|(& digit, _)| digit.pan_u32 ())
				.sum::<u32> ()
		)
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		if input.digits.len () % 2 != 0 {
			return Err ("Must have an even number of digits".into ());
		}
		Ok (
			input.digits.iter ().take (input.digits.len () / 2)
				.zip (input.digits.iter ().skip (input.digits.len () / 2))
				.filter (|& (& digit_0, & digit_1)| digit_0 == digit_1)
				.map (|(& digit, _)| digit.pan_u32 () * 2)
				.sum::<u32> ()
		)
	}

}

pub mod model {

	use super::*;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub digits: Vec <u8>,
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Self> {
			if input.len () != 1 { return Err ("Input must be exactly one line".into ()) }
			let digits = input [0].chars ()
				.map (|ch| ch.to_digit (10)
					.map (u32::pan_u8)
					.ok_or_else (|| format! ("Not a digit: {}", ch).into ()))
				.collect::<GenResult <_>> () ?;
			Ok (Self { digits })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_one (& ["1122"]));
		assert_eq_ok! ("4", puzzle.part_one (& ["1111"]));
		assert_eq_ok! ("0", puzzle.part_one (& ["1234"]));
		assert_eq_ok! ("9", puzzle.part_one (& ["91212129"]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("6", puzzle.part_two (& ["1212"]));
		assert_eq_ok! ("0", puzzle.part_two (& ["1221"]));
		assert_eq_ok! ("4", puzzle.part_two (& ["123425"]));
		assert_eq_ok! ("12", puzzle.part_two (& ["123123"]));
		assert_eq_ok! ("4", puzzle.part_two (& ["12131415"]));
	}

}
