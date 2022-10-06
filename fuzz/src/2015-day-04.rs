#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_2015::day_04::*;
use aoc_common::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.num_zeros_one = cmp::min (input.params.num_zeros_one, 2);
		input.params.num_zeros_two = cmp::min (input.params.num_zeros_two, 3);
		input.params.max_threads = 1;
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
