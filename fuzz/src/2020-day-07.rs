#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_2020::day_07::*;
use aoc_common::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.max_iters_one = cmp::min (input.params.max_iters_one, 400);
		input.params.max_iters_two = cmp::min (input.params.max_iters_two, 200);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});