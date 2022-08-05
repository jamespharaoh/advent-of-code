//! Advent of Code 2016: Day 15: Timing is Everything
//!
//! [https://adventofcode.com/2016/day/15](https://adventofcode.com/2016/day/15)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Timing is Everything";
	year = 2016;
	day = 15;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Disc;
	use model::Input;

	/// Implementation for part one
	///
	/// Simply calls [`calc_result`].
	///
	pub fn part_one (input: & Input) -> GenResult <u64> {
		calc_result (input)
	}

	/// Implementation for part two
	///
	/// Modifies the provided [`Input`] to add one extra disc with `11` positions starting at
	/// position `0`. Then hands over to [`calc_result`] for the main logic.
	///
	pub fn part_two (input: & Input) -> GenResult <u64> {
		let input = Input {
			discs: input.discs.iter ().cloned ()
				.chain (iter::once (Disc {
					delay: input.discs.len ().as_u8 () + 1,
					num_posns: 11,
					start_pos: 0,
				}))
				.collect (),
		};
		calc_result (& input)
	}

	/// Main logic for both parts, take a list of discs and calculate the start time
	///
	/// We iterate over each disc, finding the first start time at which all discs up to this point
	/// will be in the right positions. We start off with start time zero and add one each time,
	/// but we also increase the stepping each time to the lowest common multiplier of all discs so
	/// far.
	/// 
	fn calc_result (input: & Input) -> GenResult <u64> {
		let (time, _) = input.discs.iter ()
			.fold (Ok ((0_u64, 1_u64)), |state, disc|
				state.and_then (|(time, step)| Ok::<_, GenError> ((
					(time .. )
						.step_by (step.as_usize ())
						.take (disc.num_posns.as_usize ())
						.find (|time|
							(time + disc.delay.as_u64 () + disc.start_pos.as_u64 ())
								% disc.num_posns.as_u64 () == 0)
						.ok_or ("No solution found") ?,
					(step .. )
						.step_by (step.as_usize ())
						.find (|step| step % disc.num_posns.as_u64 () == 0)
						.unwrap (),
				)))) ?;
		Ok (time)
	}

}

pub mod model {

	use super::*;
	use parser::*;

	#[ derive (Clone, Debug, Default, Eq, PartialEq) ]
	pub struct Input {
		pub discs: Vec <Disc>
	}

	#[ derive (Clone, Debug, Default, Eq, PartialEq) ]
	pub struct Disc {
		pub delay: u8,
		pub num_posns: u8,
		pub start_pos: u8,
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Self> {
			#[ allow (clippy::redundant_closure_for_method_calls) ]
			let discs: Vec <Disc> = input.iter ()
				.enumerate ()
				.map (|(line_idx, line)|
					Parser::wrap (line, |parser| parser.item ())
						.map_parse_err (|col_idx|
							format! ("Invalid input: line {}: col {}: {}",
								line_idx + 1, col_idx + 1, line)))
				.collect::<GenResult <_>> () ?;
			if discs.is_empty () { return Err ("Must provide at least one disc".into ()) }
			Ok (Self { discs })
		}
	}

	impl FromParser for Disc {
		fn from_parser (parser: & mut Parser) -> ParseResult <Self> {
			let delay = parser.expect ("Disc #") ?.int () ?;
			if delay > 100 { return Err ("Max disc delay is 100".into ()) }
			let num_posns = parser.expect (" has ") ?.int () ?;
			if num_posns < 1 { return Err ("Min disc positions is 1".into ()) }
			if num_posns > 100 { return Err ("Max disc positions is 100".into ()) }
			let start_pos = parser.expect (" positions; at time=0, it is at position ") ?.int () ?;
			if start_pos >= num_posns { return Err ("Disc start pos out of range".into ()) }
			parser.expect (".") ?.end () ?;
			Ok (Self { delay, num_posns, start_pos })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"Disc #1 has 5 positions; at time=0, it is at position 4.",
		"Disc #2 has 2 positions; at time=0, it is at position 1.",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("5", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("85", puzzle.part_two (EXAMPLE));
	}

}
