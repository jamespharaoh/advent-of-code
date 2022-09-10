#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_common::*;
use aoc_2019::day_12::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.num_steps_one = cmp::min (input.params.num_steps_one, 1_000);
		input.params.num_steps_two = cmp::min (input.params.num_steps_two, 500_000);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
