//! Advent of Code 2017: Day 6: Memory Reallocation
//!
//! [https://adventofcode.com/2017/day/6](https://adventofcode.com/2017/day/6)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Memory Reallocation";
	year = 2017;
	day = 6;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Banks;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		let (_, cycle) = calc_result (input) ?;
		Ok (cycle)
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		let (cycle_0, cycle_1) = calc_result (input) ?;
		Ok (cycle_1 - cycle_0)
	}

	fn calc_result (input: & Input) -> GenResult <(u32, u32)> {
		let mut banks = input.banks.clone ();
		if banks.len () < 2 { return Err ("Must have at least two banks".into ()) }
		if banks.iter ().any (|& val| val > 24) {
			return Err ("Max size of memory in one bank is 24".into ());
		}
		let mut seen: HashMap <Banks, u32> = HashMap::new ();
		for cycle in 0 .. {
			if let Some (prev_cycle) = seen.insert (banks.clone (), cycle) {
				return Ok ((prev_cycle, cycle));
			}
			let (mut idx, & val) = banks.iter ()
				.enumerate ()
				.max_by_key (|& (idx, & val)| (val, cmp::Reverse (idx)))
				.unwrap ();
			banks [idx] = 0;
			for _ in 0 .. val {
				idx += 1;
				if idx == banks.len () { idx = 0; }
				banks [idx] += 1;
			}
		}
		unreachable! ();
	}

}

pub mod model {

	use super::*;

	pub type Banks = ArrayVec <u8, 16>;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub banks: Banks,
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Self> {
			if input.len () != 1 { return Err ("Input must be exactly one line".into ()) }
			let banks =
				Parser::wrap (input [0], |parser| {
					let mut banks = Banks::new ();
					loop {
						parser.skip_whitespace ( .. ) ?;
						if parser.peek_rest ().is_empty () { break }
						if banks.is_full () { return Err ("Max sixteen memory banks".into ()) }
						banks.push (parser.uint () ?);
					}
					Ok (banks)
				}).map_parse_err (|_, col_idx|
					format! ("Invalid input: col {}: {}", col_idx + 1, input [0])) ?;
			Ok (Self { banks })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [ "0 2 7 0" ];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("5", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("4", puzzle.part_two (EXAMPLE));
	}

}
