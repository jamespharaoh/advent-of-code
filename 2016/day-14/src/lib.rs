//! Advent of Code 2016: Day 14: One-Time Pad
//!
//! [https://adventofcode.com/2016/day/14](https://adventofcode.com/2016/day/14)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_md5 as md5;
use aoc_parallel as parallel;
use parallel::ThreadMap;

puzzle_info! {
	name = "One-Time Pad";
	year = 2016;
	day = 14;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		Ok (key_indexes (input, 0)
			.nth (input.num_keys.as_usize () - 1)
			.unwrap ())
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		Ok (key_indexes (input, input.hash_reps)
			.nth (input.num_keys.as_usize () - 1)
			.unwrap ())
	}

	struct KeyIndexIter <HashesIter: Iterator> {
		hashes: MultiPeek <HashesIter>,
		idx: u32,
		num_next: u32,
	}

	#[ inline ]
	fn key_indexes (
		input: & Input,
		hash_reps: u32,
	) -> KeyIndexIter <impl Iterator <Item = [u8; 32]>> {
		KeyIndexIter {
			hashes: hashes_iter (input, hash_reps).multipeek (),
			idx: 0,
			num_next: input.num_next,
		}
	}

	impl <HashesIter> Iterator for KeyIndexIter <HashesIter>
		where HashesIter: Iterator <Item = [u8; 32]> {

		type Item = u32;

		#[ inline ]
		fn next (& mut self) -> Option <u32> {
			loop {
				let hash_0 = self.hashes.next ().unwrap ();
				if let Some (ch_0) = find_triple (& hash_0) {
					for _ in 0 .. self.num_next.as_usize () {
						if ! has_quintuple (self.hashes.peek ().unwrap (), ch_0) { continue }
						self.idx += 1;
						return Some (self.idx - 1);
					}
				}
				self.idx += 1;
			}
		}

	}

	fn find_triple (hash: & [u8; 32]) -> Option <u8> {
		hash.iter_vals ()
			.tuple_windows::<(_, _, _)> ()
			.filter (|& (a, b, c)| a == b && a == c)
			.map (|(a, _, _)| a)
			.next ()
	}

	fn has_quintuple (hash: & [u8; 32], ch_0: u8) -> bool {
		hash.iter_vals ()
			.fold ((false, 0_u32), |(matched, count), ch_1|
				if matched || (ch_0 == ch_1 && count == 4){ (true, 0) }
				else if ch_0 == ch_1 { (false, count + 1) }
				else { (false, 0) }
			).0
	}

	fn hashes_iter (
		input: & Input,
		hash_reps: u32,
	) -> impl Iterator <Item = [u8; 32]> {
		const BATCH_SIZE: usize = 1000;
		let salt = input.salt.clone ();
		let num_start_iter = (0_i32 .. ).step_by (BATCH_SIZE);
		let map_fn = move |num_start| {
			let mut buffer = salt.clone ();
			(num_start .. )
				.take (BATCH_SIZE)
				.map (|num| {
					buffer.truncate (salt.len ());
					write! (& mut buffer, "{}", num).unwrap ();
					stretched_hash (& buffer, hash_reps)
				})
				.collect::<Vec <_>> ()
		};
		ThreadMap::start (num_start_iter, map_fn, get_num_threads (input)).flatten ()
	}

	fn get_num_threads (input: & Input) -> usize {
		if input.max_threads == 1 { return input.max_threads }
		parallel::num_cpus ().unwrap_or (input.max_threads)
	}

	#[ inline ]
	fn stretched_hash (input: & str, hash_reps: u32) -> [u8; 32] {
		let mut hasher = md5::MD5::new ();
		hasher.update (input.as_bytes ());
		let mut hash = hasher.finish_reset ();
		for _ in 0 .. hash_reps {
			hasher.update (& hash.as_hex_bytes ());
			hash = hasher.finish_reset ();
		}
		hash.as_hex_bytes ()
	}

}

pub mod model {

	use super::*;
	use parser::*;

	#[ derive (Clone, Debug, Default, Eq, PartialEq) ]
	pub struct Input {
		pub salt: String,
		pub num_keys: u32,
		pub num_next: u32,
		pub hash_reps: u32,
		pub max_threads: usize,
	}

	impl Input {
		pub fn parse (mut input: & [& str]) -> GenResult <Self> {

			let num_keys = parser::input_param (& mut input, "NUM_KEYS=", 64_u32) ?;
			if num_keys < 1 { return Err ("Number of keys must be at least one".into ()) }
			if num_keys > 100 { return Err ("Number of keys must be 100 or less".into ()) }

			let num_next = parser::input_param (& mut input, "NUM_NEXT=", 1000_u32) ?;
			if num_next < 1 { return Err ("Number of next hashes must be at least one".into ()) }
			if num_next > 2000 { return Err ("Number of next hashes must be 2000 or less".into ()) }

			let hash_reps = parser::input_param (& mut input, "HASH_REPS=", 2016_u32) ?;
			if hash_reps < 1 { return Err ("Hash reps must be at least one".into ()) }
			if hash_reps > 3000 { return Err ("Hash reps must be 3000 or less".into ()) }

			let max_threads = parser::input_param (& mut input, "MAX_THREADS=", usize::MAX) ?;
			if hash_reps < 1 { return Err ("Hash reps must be at least one".into ()) }

			if input.len () != 1 { return Err ("Invalid input: more than one line".into ()) }

			let salt =
				Parser::wrap (input [0], |parser| Ok (parser.rest ().to_owned ()))
					.map_parse_err (|col_idx|
						format! ("Invalid input: col {}: {}", col_idx + 1, input [0])) ?;
			if salt.is_empty () { return Err ("Salt must be at least one character".into ()) }

			Ok (Self { salt, num_keys, num_next, hash_reps, max_threads })

		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"NUM_KEYS=16",
		"NUM_NEXT=1000",
		"HASH_REPS=16",
		"MAX_THREADS=2",
		"abc",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("1144", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("4684", puzzle.part_two (EXAMPLE));
	}

}
