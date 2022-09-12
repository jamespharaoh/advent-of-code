#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_2019::day_16::*;
use aoc_common::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.num_iters = cmp::min (input.params.num_iters, 10);
		input.params.max_signal = cmp::min (input.params.max_signal, 100_000);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
