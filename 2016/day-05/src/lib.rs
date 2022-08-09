//! Advent of Code 2016: Day 5: How About a Nice Game of Chess?
//!
//! [https://adventofcode.com/2016/day/5](https://adventofcode.com/2016/day/5)
//!
//! # Input
//!
//! Any string, used as a cryptographic salt. Only a single line is used. For testing purposes, we
//! also accept a prefix line in the form `"NUM_ZEROS=n"` where n is the number of zeros to match
//! at the start of the generated hash.
//!
//! # Part one
//!
//! Construct a password by searching for integers which can be appended to the input string which
//! give an md5 hash having at least 5 zeros at the start of its hex representation. The password
//! is built by appending the 6th hex digit each time. Once eight characters are obtained, the
//! password is complete.
//!
//! # Part two
//!
//! The sixth character of the hex hash now represents the position of the character in the
//! password, and the seventh is the character itself.
//!
//! # Algorithm
//!
//! We use the md5 implementation in [`aoc_common`]. Iterate over integers to append until we find
//! a match and build up the password.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_md5 as md5;
use md5::md5_hash;

puzzle_info! {
	name = "How About a Nice Game of Chess?";
	year = 2016;
	day = 5;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;
	use nums::IntConv;

	pub fn part_one (input: & Input) -> GenResult <String> {
		let mut password = String::new ();
		let mut attempt = input.door_id.clone ();
		for idx in 0_u32 .. {
			if password.len () == 8 { break }
			attempt.truncate (input.door_id.len ());
			write! (& mut attempt, "{}", idx).unwrap ();
			let hash = md5_hash (attempt.as_bytes ());
			if hash.num_zeros () < input.num_zeros { continue }
			let hash_str = hash.to_string ();
			password.push (hash_str.chars ().nth (5).unwrap ());
		}
		Ok (password)
	}

	pub fn part_two (input: & Input) -> GenResult <String> {
		let mut password = [None; 8];
		let mut attempt = input.door_id.clone ();
		for idx in 0_u32 .. {
			if ! password.iter ().any (Option::is_none) { break }
			attempt.truncate (input.door_id.len ());
			write! (& mut attempt, "{}", idx).unwrap ();
			let hash = md5_hash (attempt.as_bytes ());
			if hash.num_zeros () < input.num_zeros { continue }
			let hash_str = hash.to_string ();
			let pos =
				hash_str.chars ()
					.nth (5)
					.unwrap ()
					.to_digit (16)
					.unwrap ()
					.as_usize ();
			if 8 <= pos { continue }
			if password [pos].is_some () { continue }
			let ch = hash_str.chars ().nth (6).unwrap ();
			password [pos] = Some (ch);
		}
		let password =
			password.iter_vals ()
				.map (Option::unwrap)
				.collect ();
		Ok (password)
	}

}

pub mod model {

	use super::*;

	pub struct Input {
		pub door_id: String,
		pub num_zeros: u8,
	}

	impl Input {
		pub fn parse (mut input: & [& str]) -> GenResult <Self> {
			let num_zeros =
				input [0].strip_prefix ("NUM_ZEROS=")
					.map_or (Ok (5), |num_zeros| {
						input = & input [1 .. ];
						num_zeros.parse ()
					}) ?;
			if input.len () != 1 { Err ("Invalid input") ?; }
			let door_id = input [0].to_owned ();
			Ok (Self { door_id, num_zeros })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [ "NUM_ZEROS=1", "abc" ];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("0500f456", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("36790f5e", puzzle.part_two (EXAMPLE));
	}

}
