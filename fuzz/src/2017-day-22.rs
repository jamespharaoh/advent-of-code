#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_common::*;
use aoc_2017::day_22::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.iters_one = cmp::min (input.params.iters_one, 1_000);
		input.params.iters_two = cmp::min (input.params.iters_two, 10_000);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
