#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_2016::day_14::*;
use aoc_common::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.num_keys = cmp::min (input.params.num_keys, 2);
		input.params.hash_reps = cmp::min (input.params.hash_reps, 2);
		input.params.max_threads = cmp::min (input.params.max_threads, 1);
		input.params.batch_size = cmp::min (input.params.batch_size, 100);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
