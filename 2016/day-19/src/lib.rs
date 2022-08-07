//! Advent of Code 2016: Day 19: An Elephant Named Joseph
//!
//! [https://adventofcode.com/2016/day/19](https://adventofcode.com/2016/day/19)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "An Elephant Named Joseph";
	year = 2016;
	day = 19;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		if input.num_elves < 2 { return Err ("Must have at least two elves".into ()) }
		let mut first_with = 0;
		let mut sep = 1;
		let mut rem = input.num_elves;
		let mut take = false;
		while rem > 1 {
			let odd = (rem & 1) == 1;
			if ! take && odd { rem = (rem + 1) / 2; } else { rem /= 2; }
			if take { first_with += sep; }
			if odd { take = ! take; }
			sep = u32::mul_2 (sep, 2) ?;
		}
		Ok (first_with + 1)
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		if input.num_elves < 2 { return Err ("Must have at least two elves".into ()) }
		let mut elves =
			((input.num_elves + 2) / 2 ..= input.num_elves)
				.chain (1 .. (input.num_elves + 2) / 2)
				.collect::<Vec <_>> ();
		let mut seq = 1 + (input.num_elves) % 2;
		while elves.len () > 3 {
			let next_seq = (seq + elves.len ().as_u32 ()) % 3;
			elves = elves.iter_vals ()
				.scan (seq, |state, elf| {
					let seq = * state;
					* state += 1;
					if * state == 3 { * state = 0; }
					Some ((seq, elf))
				})
				.filter (|& (seq, _)| seq == 0)
				.map (|(_, elf)| elf)
				.collect ();
			seq = next_seq;
		}
		while elves.len () > 1 {
			let elf = elves.remove (0);
			if seq == 0 { elves.push (elf); }
			seq = (seq + 1) % 3;
		}
		Ok (elves [0])
	}

}

pub mod model {

	use super::*;
	use parser::*;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub num_elves: u32,
	}

	impl Input {

		pub fn parse (input: & [& str]) -> GenResult <Self> {
			if input.len () != 1 { return Err ("Input must have exactly one line".into ()) }
			#[ allow (clippy::redundant_closure_for_method_calls) ]
			let num_elves = Parser::wrap (input [0], |parser| parser.int ())
				.map_parse_err (|col_idx|
					format! ("Invalid input: col {}: {}", col_idx + 1, input [0])) ?;
			Ok (Self { num_elves })
		}

	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		for (input, expected) in [
			(2, 1), (3, 3), (4, 1), (5, 3), (6, 5), (7, 7), (8, 1), (9, 3), (10, 5), (11, 7),
			(12, 9), (13, 11), (14, 13), (15, 15), (16, 1), (17, 3), (18, 5), (19, 7), (20, 9),
		].into_iter () {
			assert_eq_ok! (expected.to_string (), puzzle.part_one (& [& input.to_string ()]));
		}
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		for (input, expected) in [
			(2, 1), (3, 3), (4, 1), (5, 2), (6, 3), (7, 5), (8, 7), (9, 9), (10, 1), (11, 2),
			(12, 3), (13, 4), (14, 5), (15, 6), (16, 7), (17, 8), (18, 9), (19, 11), (20, 13),
		].into_iter () {
			assert_eq_ok! (expected.to_string (), puzzle.part_two (& [& input.to_string ()]));
		}
	}

}
