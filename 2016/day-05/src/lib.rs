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
//! a match and build up the password. To make things faster we use [`aoc_parallel::ThreadMap`] to
//! generate hashes parallely in separate threads.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_parallel as parallel;
use aoc_md5 as md5;
use parallel::ThreadMap;
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

	pub fn part_one (input: & Input) -> GenResult <String> {
		let mut password = String::new ();
		for hash in iter_hashes (input) {
			if password.len () == 8 { break }
			let hash_str = hash.to_string ();
			password.push (hash_str.chars ().nth (5).unwrap ());
		}
		Ok (password)
	}

	pub fn part_two (input: & Input) -> GenResult <String> {
		let mut password = [None; 8];
		for hash in iter_hashes (input) {
			if ! password.iter ().any (Option::is_none) { break }
			let hash_str = hash.to_string ();
			let pos =
				hash_str.chars ()
					.nth (5)
					.unwrap ()
					.to_digit (16)
					.unwrap ()
					.pan_usize ();
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

	fn iter_hashes (input: & Input) -> impl Iterator <Item = md5::Output> {
		const BATCH_SIZE: usize = 1000;
		let door_id: Arc <str> = Arc::from (input.door_id.as_str ());
		let inner_iter = (0_u32 .. ).step_by (BATCH_SIZE);
		let num_zeros = input.num_zeros;
		let map_fn = move |num_start| {
			let mut hashes = Vec::new ();
			let mut buffer = door_id.deref ().to_owned ();
			for num in (num_start .. ).take (BATCH_SIZE) {
				buffer.truncate (door_id.len ());
				write! (buffer, "{}", num).unwrap ();
				let hash = md5_hash (buffer.as_bytes ());
				if hash.num_zeros () < num_zeros { continue }
				hashes.push (hash);
			}
			hashes
		};
		ThreadMap::start (inner_iter, map_fn, get_max_threads (input)).flatten ()
	}

	fn get_max_threads (input: & Input) -> usize {
		if input.max_threads == 1 { return 1 }
		cmp::min (input.max_threads, parallel::num_cpus ().unwrap_or (1))
	}

}

pub mod model {

	use super::*;

	pub struct Input {
		pub door_id: String,
		pub num_zeros: u8,
		pub max_threads: usize,
	}

	impl Input {
		pub fn parse (mut input: & [& str]) -> GenResult <Self> {
			let num_zeros = parser::input_param (& mut input, "NUM_ZEROS=", 5) ?;
			let max_threads = parser::input_param (& mut input, "MAX_THREADS=", usize::MAX) ?;
			if input.len () != 1 { Err ("Invalid input") ?; }
			let door_id = input [0].to_owned ();
			Ok (Self { door_id, num_zeros, max_threads })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [ "NUM_ZEROS=1", "MAX_THREADS=1", "abc" ];

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
