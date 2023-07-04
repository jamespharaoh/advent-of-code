#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_2018::day_22::*;
use aoc_common::*;

use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.max_target = cmp::min (input.params.max_target, 50);
		input.params.max_mins = cmp::min (input.params.max_mins, 100);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});