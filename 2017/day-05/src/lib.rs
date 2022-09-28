//! Advent of Code 2017: Day 5: A Maze of Twisty Trampolines, All Alike
//!
//! [https://adventofcode.com/2017/day/5](https://adventofcode.com/2017/day/5)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "A Maze of Twisty Trampolines, All Alike";
	year = 2017;
	day = 5;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;
	use model::Tramp;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		let mut tramps = input.trampolines.clone ();
		let mut offset = Tramp::ZERO;
		let mut count = 0_u32;
		while offset >= Tramp::ZERO && offset.pan_usize () < tramps.len () {
			let tramp = & mut tramps [offset.pan_usize ()];
			offset += * tramp;
			* tramp += Tramp::ONE;
			count += 1_u32;
		}
		Ok (count)
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		const THREE: Tramp = 3;
		let mut tramps = input.trampolines.clone ();
		let mut offset = Tramp::ZERO;
		let mut count = 0_u32;
		while offset >= Tramp::ZERO && offset.pan_usize () < tramps.len () {
			let tramp = & mut tramps [offset.pan_usize ()];
			offset += * tramp;
			* tramp = if * tramp >= THREE { * tramp - Tramp::ONE } else { * tramp + Tramp::ONE };
			count += 1_u32;
		}
		Ok (count)
	}

}

pub mod model {

	use super::*;

	pub type Tramp = i16;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub trampolines: Vec <Tramp>,
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Self> {
			let trampolines = input.iter ()
				.enumerate ()
				.map (|(line_idx, line)| line.parse ()
					.map_err (|_err|
						format! ("Invalid input: line {}: {}", line_idx + 1, line).into ()))
				.collect::<GenResult <Vec <Tramp>>> () ?;
			Ok (Self { trampolines })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"0",
		"3",
		"0",
		"1",
		"-3",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("5", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("10", puzzle.part_two (EXAMPLE));
	}

}
