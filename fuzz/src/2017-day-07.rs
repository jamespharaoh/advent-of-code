#![ no_main ]

use libfuzzer_sys::fuzz_mutator;
use libfuzzer_sys::fuzz_target;
use libfuzzer_sys::fuzzer_mutate;
use rand::prelude::*;

use aoc_common::*;
use aoc_2017::day_07::*;
use input::Input;

const PROB_CUSTOM_FIRST: f64 = 0.7;
const PROB_BUILTIN_AFTER: f64 = 0.3;

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
	if rng.gen_bool (PROB_CUSTOM_FIRST) {
		if let Some (new_size) = mutator::main (data, size, max_size, & mut rng) {
			num_mutations += 1;
			size = new_size;
		}
	}
	while num_mutations == 0 || rng.gen_bool (PROB_BUILTIN_AFTER) {
		size = fuzzer_mutate (data, size, max_size);
		num_mutations += 1;
	}
	size
});

mod mutator {

	use super::*;
	use input::Prog;
	use model::ProgInfo;

	pub fn main (
		data: & mut [u8],
		size: usize,
		max_size: usize,
		rng: & mut StdRng,
	) -> Option <usize> {

		let output_str = {

			let input_str = str::from_utf8 (& data [ .. size]).ok () ?;
			let input_vec: Vec <& str> = input_str.trim ().split ('\n').collect ();
			let input = Input::parse (& input_vec).ok () ?;

			let output_dat = if rng.gen_bool (0.3) {
				let mut output_dat = input;
				mutate_input (& mut output_dat, rng);
				output_dat
			} else {
				let mut root = ProgInfo::build (& input).ok () ?;
				mutate_model (& mut root, rng);
				let mut output_dat = Input { progs: Vec::new () };
				root.write_input (& mut output_dat.progs);
				output_dat.progs.shuffle (rng);
				output_dat
			};

			let mut output_str = String::new ();
			output_dat.write_str (& mut output_str).unwrap ();
			output_str

		};

		if max_size < output_str.len () { return None }
		(& mut data [0 .. output_str.len ()]).copy_from_slice (output_str.as_bytes ());
		Some (output_str.len ())

	}

	fn fix_weights (prog: & mut ProgInfo, rng: & mut StdRng, target: u32) -> bool {
		if ! prog.holds.is_empty () {
			let child_target = prog.holds.choose (rng).unwrap ().total_weight;
			prog.holds_weight = 0;
			for held in prog.holds.iter_mut () {
				if ! fix_weights (held, rng, child_target) { return false }
				prog.holds_weight += child_target;
			}
			if target <= prog.holds_weight { return false }
			prog.prog_weight = target - prog.holds_weight;
		} else {
			prog.prog_weight = target;
		}
		true
	}

	fn visit (
		prog: & mut ProgInfo,
		rng: & mut StdRng,
		mut_fn: fn (& mut ProgInfo, & mut StdRng),
	) {
		fn recurse (
			prog: & mut ProgInfo,
			cur_idx: & mut u32,
			mod_idx: u32,
			rng: & mut StdRng,
			mut_fn: fn (& mut ProgInfo, & mut StdRng),
		) {
			if * cur_idx == mod_idx { mut_fn (prog, rng); }
			* cur_idx += 1;
			prog.holds_weight = 0;
			for held in prog.holds.iter_mut () {
				recurse (held, cur_idx, mod_idx, rng, mut_fn);
				prog.holds_weight += held.total_weight;
			}
			prog.total_weight = prog.prog_weight + prog.holds_weight;
		}
		let mod_idx = rng.gen_range (0 .. prog.nested_len ().pan_u32 ());
		recurse (prog, & mut 0, mod_idx, rng, mut_fn);
	}

	fn mutate_model (root: & mut ProgInfo, rng: & mut StdRng) {
		match rng.gen_range (0 .. 5) {

			// change the name of a random programme

			0 => visit (root, rng, |prog, rng| {
				prog.name = crate::InpStr::alloc (make_name (rng));
			}),

			// remove a random programme and all the programmes it holds

			1 => visit (root, rng, |prog, rng| {
				if prog.holds.is_empty () { return }
				prog.holds.remove (rng.gen_range (0 .. prog.holds.len ()));
			}),

			// replace a random programme with one of its children

			2 => visit (root, rng, |prog, rng| {
				if prog.holds.is_empty () { return }
				let mut child = prog.holds.remove (rng.gen_range (0 .. prog.holds.len ()));
				mem::swap (prog, & mut child);
			}),

			// insert a new programme in a random location

			3 => visit (root, rng, |prog, rng| {
				if prog.holds.len () == 7 { return }
				prog.holds.insert (rng.gen_range (0 ..= prog.holds.len ()), ProgInfo {
					name: InpStr::alloc (make_name (rng)),
					holds: default (),
					prog_weight: rng.gen_range (1_u32 .. 999),
					holds_weight: 0,
					total_weight: 0,
				});
			}),

			// try and balance all programme weights, then change one randomly

			4 => {
				for total_weight in (root.total_weight .. ).take (100) {
					if fix_weights (root, rng, total_weight) { break }
				}
				visit (root, rng, |prog, rng| {
					prog.prog_weight = rng.gen_range (1_u32 .. 100);
				});
			},

			_ => unreachable! (),

		}
	}

	fn mutate_input (input: & mut Input, rng: & mut StdRng) {
		match rng.gen_range (0 .. 2) {

			// remove a random prog

			0 => {
				let idx = rng.gen_range (0 .. input.progs.len ());
				input.progs.remove (idx);
			},

			// insert a random prog

			1 => {
				let idx = rng.gen_range (0 .. input.progs.len ());
				input.progs.insert (idx, make_input_prog (rng));
			},

			_ => unreachable! (),

		}
	}

	fn make_input_prog <'inp> (rng: & mut StdRng) -> Prog <'inp> {
		let num_holds = rng.gen_range (0 ..= 7);
		let holds =
			iter::from_fn (|| Some (InpStr::alloc (& make_name (rng))))
				.take (num_holds)
				.collect ();
		Prog {
			name: InpStr::alloc (& make_name (rng)),
			weight: rng.gen_range (1 .. 100),
			holds,
		}
	}

	fn make_name (rng: & mut StdRng) -> String {
		static NAME_LENGTHS: & [u8] = & [
			1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3,
			3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 7, 8, 9,
		];
		let name_len = NAME_LENGTHS.choose (rng).unwrap ().pan_usize ();
		iter::from_fn (|| Some (rng.gen_range ('a' ..= 'z')))
			.take (name_len)
			.collect::<String> ()
	}

}
