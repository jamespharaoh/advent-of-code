#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_common::*;
use aoc_2017::day_15::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse (& input_vec) {
		input.params.reps_one = cmp::min (input.params.reps_one, 4_000_000);
		input.params.reps_two = cmp::min (input.params.reps_two, 500_000);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
