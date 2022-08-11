#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_common::*;
use aoc_2017::day_10::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse (& input_vec) {
		input.params.string_size = cmp::min (input.params.string_size, 512);
		input.params.rounds_one = cmp::min (input.params.rounds_one, 4);
		input.params.rounds_two = cmp::min (input.params.rounds_two, 16);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
