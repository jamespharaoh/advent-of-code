#![ no_main ]

use aoc_2015::day_09::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim ().split ('\n').collect ();
	if let Ok (input) = model::parse_input (& input_vec) {
		let _ = logic::part_one (input.clone ());
		let _ = logic::part_two (input);
	}
});
