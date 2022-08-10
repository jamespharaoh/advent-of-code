//! Advent of Code 2017: Day 4: High-Entropy Passphrases
//!
//! [https://adventofcode.com/2017/day/4](https://adventofcode.com/2017/day/4)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "High-Entropy Passphrases";
	year = 2017;
	day = 4;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;

	pub fn part_one (input: & Input <'_>) -> GenResult <u32> {
		Ok (
			input.passphrases.iter ()
				.filter (|passphrase| ! passphrase.iter ()
					.tuple_combinations::<(_, _)> ()
					.any (|(left, right)| left == right))
				.count ()
				.as_u32 ()
		)
	}

	pub fn part_two (input: & Input <'_>) -> GenResult <u32> {
		let mut buffer_pool: Vec <Vec <char>> = Vec::new ();
		let mut words: Vec <Vec <char>> = Vec::new ();
		Ok (
			input.passphrases.iter ()
				.filter (|passphrase| {
					words.extend (
						passphrase.iter ()
							.map (|word| {
								let mut buffer = buffer_pool.pop ().unwrap_or_default ();
								buffer.clear ();
								buffer.extend (word.chars ());
								buffer.sort ();
								buffer
							}));
					let result =
						! words.iter ()
							.tuple_combinations::<(_, _)> ()
							.any (|(left, right)| left == right);
					buffer_pool.append (& mut words);
					result
				})
				.count ()
				.as_u32 ()
		)
	}

}

pub mod model {

	use super::*;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input <'inp> {
		pub passphrases: Vec <ArrayVec <& 'inp str, 16>>,
	}

	impl <'inp> Input <'inp> {
		pub fn parse (input: & [& 'inp str]) -> GenResult <Self> {
			let passphrases = input.iter ().enumerate ()
				.map (|(line_idx, line)|
					(|| {
						if line.split (' ').count () > 16 {
							return Err ("Can't have more than sixteen words");
						}
						let words: ArrayVec <& str, 16> = line.split (' ').collect ();
						if words.iter ().any (|word| word.is_empty ()) {
							return Err ("Can't have empty word");
						}
						if words.iter ().any (|word| word.chars ()
								.any (|ch| ! ch.is_ascii_lowercase ())) {
							return Err ("Words must be ASCII lowercase");
						}
						Ok (words)
					}) ().map_err (|err| format! (
						"Invalid input: line {}: {}: {}", line_idx + 1, err, line).into ()))
				.collect::<GenResult <_>> () ?;
			Ok (Self { passphrases })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_ONE: & [& str] = & [
		"aa bb cc dd ee",
		"aa bb cc dd aa",
		"aa bb cc dd aaa",
	];

	const EXAMPLE_TWO: & [& str] = & [
		"abcde fghij",
		"abcde xyz ecdab",
		"a ab abc abd abf abj",
		"iiii oiii ooii oooi oooo",
		"oiii ioii iioi iiio",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("2", puzzle.part_one (EXAMPLE_ONE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_two (EXAMPLE_TWO));
	}

}
