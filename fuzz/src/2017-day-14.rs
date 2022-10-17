#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_common::*;
use aoc_2017::day_14::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.num_rounds = cmp::min (input.params.num_rounds, 4);
		input.params.num_rows = cmp::min (input.params.num_rows, 4);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
