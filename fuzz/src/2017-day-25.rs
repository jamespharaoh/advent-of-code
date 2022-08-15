#![ no_main ]

use libfuzzer_sys::fuzz_mutator;
use libfuzzer_sys::fuzz_target;
use libfuzzer_sys::fuzzer_mutate;
use rand::prelude::*;

use aoc_common::*;
use aoc_2017::day_25::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse (& input_vec) {
		input.num_steps = cmp::min (input.num_steps, 1_000_000);
		let _ = logic::part_one (& input);
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
	use model::Dir;
	use model::Slot;
	use model::State;

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

		const TRANSFORMS: & [(u32, u32, fn (& mut Input, & mut StdRng) -> Option <()>)] = & [
			(100, 1, modify),
			(10, 3, modify),
			(100, 1, add),
			(10, 3, add),
			(100, 1, remove),
			(10, 3, remove),
			(1, 1, begin_state),
			(1, 1, num_steps),
			(1, 1, sort),
			(1, 1, shuffle),
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

		fn begin_state (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			input.begin_state = rng.gen_range ('A' ..= 'Z');
			Some (())
		}

		fn num_steps (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			input.num_steps = rng.gen_range (1 ..= 10_000_000);
			Some (())
		}

		fn modify (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			let idx = rng.gen_range (0 .. input.states.len ());
			let state = & mut input.states [idx];
			match rng.gen_range (0 .. 7) {
				0 => state.id = make_id (rng),
				1 => state.false_write = make_slot (rng),
				2 => state.false_dir = make_dir (rng),
				3 => state.false_state = make_id (rng),
				4 => state.true_write = make_slot (rng),
				5 => state.true_dir = make_dir (rng),
				6 => state.true_state = make_id (rng),
				_ => unreachable! (),
			}
			Some (())
		}

		fn add (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			let id = make_id (rng);
			let false_write = make_slot (rng);
			let false_dir = make_dir (rng);
			let false_state = make_id (rng);
			let true_write = make_slot (rng);
			let true_dir = make_dir (rng);
			let true_state = make_id (rng);
			let new_idx = rng.gen_range (0 ..= input.states.len ());
			input.states.insert (new_idx, State {
				id,
				false_write, false_dir, false_state,
				true_write, true_dir, true_state,
			});
			Some (())
		}

		pub fn remove (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			if input.states.is_empty () { return Some (()) }
			let idx = rng.gen_range (0 .. input.states.len ());
			input.states.remove (idx);
			Some (())
		}

		pub fn sort (input: & mut Input, _rng: & mut StdRng) -> Option <()> {
			input.states.sort_by_key (|state| state.id);
			Some (())
		}

		pub fn shuffle (input: & mut Input, rng: & mut StdRng) -> Option <()> {
			input.states.shuffle (rng);
			Some (())
		}

		fn make_id (rng: & mut StdRng) -> char {
			rng.gen_range ('A' ..= 'Z')
		}

		fn make_dir (rng: & mut StdRng) -> Dir {
			[ Dir::Left, Dir::Right ].choose (rng).copied ().unwrap ()
		}

		fn make_slot (rng: & mut StdRng) -> Slot {
			[ Slot::Zero, Slot::One ].choose (rng).copied ().unwrap ()
		}

	}

}
