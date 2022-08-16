#![ no_main ]

use aoc_fuzz::aoc_fuzz_mutator;
use libfuzzer_sys::fuzz_target;

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

aoc_fuzz_mutator! {

	transform_lifetimes = <'inp>;
	input_type = Input <'inp>;

	transform modify (100 * 1, 10 * 10, 1 * 100) = |input, rng| {
		let idx = rng.gen_range (0 .. input.box_ids.len ());
		let box_id = & mut input.box_ids [idx];
		if box_id.is_empty () { return Some (()) }
		let char_idx = rng.gen_range (0 .. box_id.chars ().count ());
		let new_char = rng.gen_range ('a' ..= 'z');
		input.box_ids [idx] = InpStr::alloc (
			& box_id.chars ().enumerate ()
				.map (|(idx, ch)| if idx == char_idx { new_char } else { ch })
				.collect::<String> ());
	}

	transform add (100 * 1, 10 * 10, 1 * 100) = |input, rng| {
		let len = input.box_ids.first ()
			.map (|box_id| box_id.chars ().count ())
			.unwrap_or (16);
		let box_id = InpStr::alloc (
			& iter::from_fn (|| Some (rng.gen_range ('a' ..= 'z')))
				.take (len)
				.collect::<String> ());
		let new_idx = rng.gen_range (0 ..= input.box_ids.len ());
		input.box_ids.insert (new_idx, box_id);
	}

	pub transform remove (100 * 1, 10 * 10, 1 * 100) = |input, rng| {
		if input.box_ids.is_empty () { return Some (()) }
		let idx = rng.gen_range (0 .. input.box_ids.len ());
		input.box_ids.remove (idx);
	}

	transform add_char (10) = |input, rng| {
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
	}

	transform remove_char (10) = |input, rng| {
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
	}

	transform truncate (1) = |input, rng| {
		let max_len = input.box_ids.iter ()
			.map (|box_id| box_id.chars ().count ())
			.max ()
			.unwrap_or (0);
		let len = rng.gen_range (0 ..= max_len);
		for box_id in input.box_ids.iter_mut () {
			if box_id.chars ().count () <= len { continue }
			* box_id = InpStr::alloc (& box_id.chars ().take (len).collect::<String> ());
		}
	}

	transform sort (1) = |input, _rng| {
		input.box_ids.sort ();
	}

	transform shuffle (1) = |input, rng| {
		input.box_ids.shuffle (rng);
	}

}
