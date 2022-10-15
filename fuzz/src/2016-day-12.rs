#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_2016::day_12::*;
use aoc_common::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.ops_limit = cmp::min (input.params.ops_limit, 100_000);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
