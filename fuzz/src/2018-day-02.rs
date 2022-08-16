#![ no_main ]

use libfuzzer_sys::fuzz_mutator;
use libfuzzer_sys::fuzz_target;
use libfuzzer_sys::fuzzer_mutate;
use rand::prelude::*;

use aoc_common::*;
use aoc_2018::day_02::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (input) = Input::parse (& input_vec) {
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});


fuzz_mutator! (|data: & mut [u8], size: usize, max_size: usize, seed: u32| {
	let mut rng = StdRng::seed_from_u64 (seed as u64);
	let mut size = size;
	let mut num_mutations = 0;
	if rng.gen_bool (0.7) {
		if let Some (new_size) = mutator::main (data, size, max_size, & mut rng) {
			size = new_size;
			num_mutations += 1;
		}
	}
	while num_mutations == 0 || rng.gen_bool (0.1) {
		size = fuzzer_mutate (data, size, max_size);
		num_mutations += 1;
	}
	size
});

mod mutator {

	use super::*;

	pub fn main (
		data: & mut [u8],
		size: usize,
		max_size: usize,
		rng: & mut StdRng,
	) -> Option <usize> {

		// parse input

		let input_str = str::from_utf8 (& data [0 .. size]).ok () ?;
		let input_vec: Vec <& str> = input_str.trim ().split ('\n').collect ();
		let mut input = Input::parse (& input_vec).ok () ?;

		// apply a random transform

		transforms::random (& mut input, rng);

		// convert to string, removing random lines until it is the right length

		let output_str = loop {
			let output_str = input.to_string ();
			if output_str.as_bytes ().len () <= max_size { break output_str }
			transforms::remove (& mut input, rng);
		};

		// update data, and return

		(& mut data [ .. output_str.len ()]).copy_from_slice (output_str.as_bytes ());

		Some (output_str.len ())

	}

	mod transforms {

		use super::*;

		type TransFn = for <'inp> fn (& mut Input <'inp>, & mut StdRng) -> Option <()>;

		const TRANSFORMS: & [(u32, u32, TransFn)] = & [
			(100, 1, modify),
			(10, 10, modify),
			(1, 100, modify),
			(100, 1, add),
			(10, 10, add),
			(1, 100, add),
			(100, 1, remove),
			(10, 10, remove),
			(1, 100, remove),
			(10, 1, add_char),
			(10, 1, remove_char),
			(1, 1, truncate),
			(1, 1, sort),
			(1, 1, shuffle),
		];

		pub fn random <'inp> (input: & mut Input <'inp>, rng: & mut StdRng) {
			let & (_, reps, ref trans_fn) =
				TRANSFORMS.choose_weighted (rng, |& (weight, _, _)| weight).unwrap ();
			let mut num_failure = 0;
			for _ in 0 .. reps {
				loop {
					let success = trans_fn (input, rng).is_some ();
					if success { break }
					num_failure += 1;
					if num_failure >= 10 { break }
				}
			}
		}

		fn modify <'inp> (input: & mut Input <'inp>, rng: & mut StdRng) -> Option <()> {
			let idx = rng.gen_range (0 .. input.box_ids.len ());
			let box_id = & mut input.box_ids [idx];
			if box_id.is_empty () { return Some (()) }
			let char_idx = rng.gen_range (0 .. box_id.chars ().count ());
			let new_char = rng.gen_range ('a' ..= 'z');
			input.box_ids [idx] = InpStr::alloc (
				& box_id.chars ().enumerate ()
					.map (|(idx, ch)| if idx == char_idx { new_char } else { ch })
					.collect::<String> ());
			Some (())
		}

		fn add (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			let len = input.box_ids.first ()
				.map (|box_id| box_id.chars ().count ())
				.unwrap_or (16);
			let box_id = InpStr::alloc (
				& iter::from_fn (|| Some (rng.gen_range ('a' ..= 'z')))
					.take (len)
					.collect::<String> ());
			let new_idx = rng.gen_range (0 ..= input.box_ids.len ());
			input.box_ids.insert (new_idx, box_id);
			Some (())
		}

		pub fn remove (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			if input.box_ids.is_empty () { return Some (()) }
			let idx = rng.gen_range (0 .. input.box_ids.len ());
			input.box_ids.remove (idx);
			Some (())
		}

		fn add_char (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			let len = input.box_ids.first ()
				.map (|box_id| box_id.chars ().count ())
				.unwrap_or (16);
			let char_idx = rng.gen_range (0 ..= len);
			let new_char = rng.gen_range ('a' ..= 'z');
			for box_id in input.box_ids.iter_mut () {
				* box_id = InpStr::alloc (
					& iter::empty ()
						.chain (box_id.chars ().take (char_idx))
						.chain (iter::once (new_char))
						.chain (box_id.chars ().skip (char_idx))
						.collect::<String> ());
			}
			Some (())
		}

		fn remove_char (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			let len = input.box_ids.first ()
				.map (|box_id| box_id.chars ().count ())
				.unwrap_or (16);
			let char_idx = rng.gen_range (0 ..= len);
			for box_id in input.box_ids.iter_mut () {
				* box_id = InpStr::alloc (
					& box_id.chars ().enumerate ()
						.filter (|& (idx, _)| idx != char_idx)
						.map (|(_, ch)| ch)
						.collect::<String> ());
			}
			Some (())
		}

		fn truncate <'inp> (input: & mut Input <'inp>, rng: & mut StdRng) -> Option <()> {
			let max_len = input.box_ids.iter ()
				.map (|box_id| box_id.chars ().count ())
				.max ()
				.unwrap_or (0);
			let len = rng.gen_range (0 ..= max_len);
			for box_id in input.box_ids.iter_mut () {
				if box_id.chars ().count () <= len { continue }
				* box_id = InpStr::alloc (& box_id.chars ().take (len).collect::<String> ());
			}
			Some (())
		}

		fn sort (input: & mut Input, _rng: & mut StdRng) -> Option <()> {
			input.box_ids.sort ();
			Some (())
		}

		fn shuffle (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			input.box_ids.shuffle (rng);
			Some (())
		}

	}

}
