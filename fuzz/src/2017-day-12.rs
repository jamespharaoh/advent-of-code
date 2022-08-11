#![ no_main ]

use libfuzzer_sys::fuzz_mutator;
use libfuzzer_sys::fuzz_target;
use libfuzzer_sys::fuzzer_mutate;
use rand::prelude::*;

use aoc_common::*;
use aoc_2017::day_12::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim ().split ('\n').collect ();
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
	use input::InputPipe;
	use input::MAX_PIPES;
	use model::Village;

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
		use std::iter;

		const TRANSFORMS: & [(u32, u32, fn (& mut Input, & mut StdRng) -> Option <()>)] = & [
			(1000, 1, add),
			(100, 10, add),
			(10, 100, add),
			(1000, 1, remove),
			(100, 10, remove),
			(10, 100, remove),
			(400, 10, merge),
			(40, 1, merge),
			(400, 10, split),
			(40, 1, split),
			(5, 1, sort),
			(5, 1, shuffle),
		];

		pub fn random (input: & mut Input, rng: & mut StdRng) {
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

		pub fn add (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			let left = rng.gen_range (0 ..= 199);
			let right_len = rng.gen_range (0 ..= MAX_PIPES);
			let right =
				iter::from_fn (|| Some (rng.gen_range (0 ..= 199)))
					.take (right_len)
					.collect ();
			let new_idx = rng.gen_range (0 ..= input.pipes.len ());
			input.pipes.insert (new_idx, InputPipe { left, right });
			Some (())
		}

		pub fn remove (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			if input.pipes.is_empty () { return Some (()) }
			let idx = rng.gen_range (0 .. input.pipes.len ());
			input.pipes.remove (idx);
			Some (())
		}

		pub fn sort (input: & mut Input, _rng: & mut StdRng) -> Option <()> {
			input.pipes.sort_by_key (|pipe| pipe.left);
			for pipe in input.pipes.iter_mut () { pipe.right.sort (); }
			Some (())
		}

		pub fn shuffle (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			input.pipes.shuffle (rng);
			Some (())
		}

		pub fn merge (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			let pipe_0 = & input.pipes.choose (rng) ?;
			let pipe_1 = & input.pipes.choose (rng) ?;
			let mut villages: Vec <Village> =
				[ pipe_0.left, pipe_1.left ].iter_vals ()
					.chain (pipe_0.right.iter_vals ())
					.chain (pipe_1.right.iter_vals ())
					.sorted ()
					.dedup ()
					.collect ();
			if ! (2 ..= MAX_PIPES + 1).contains (& villages.len ()) { return None }
			let left_idx = rng.gen_range (0 .. villages.len ());
			let left = villages.remove (left_idx);
			villages.shuffle (rng);
			let right = villages.into_iter ().collect ();
			let new_idx = rng.gen_range (0 .. input.pipes.len ());
			input.pipes.insert (new_idx, InputPipe { left, right });
			Some (())
		}

		fn split (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			let pipe = & input.pipes.choose (rng) ?;
			let mut villages_0: Vec <Village> =
				iter::once (pipe.left)
					.chain (pipe.right.iter_vals ())
					.sorted ()
					.dedup ()
					.collect ();
			if villages_0.len () < 2 { return None }
			let split_idx = rng.gen_range (1 ..= villages_0.len () - 1);
			let villages_1 = villages_0.split_off (split_idx);
			for mut villages in vec! [ villages_0, villages_1 ].into_iter () {
				let left_idx = rng.gen_range (0 .. villages.len ());
				let left = villages.remove (left_idx);
				if villages.is_empty () { villages = vec! [ left ]; }
				let right = villages.into_iter ().collect ();
				let new_idx = rng.gen_range (0 ..= input.pipes.len ());
				input.pipes.insert (new_idx, InputPipe { left, right });
			}
			Some (())
		}

	}

}
