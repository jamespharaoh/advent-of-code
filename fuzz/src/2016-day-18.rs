#![ no_main ]

use aoc_2016::day_18::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.split ('\n').collect ();
	if let Ok (input) = model::Input::parse (& input_vec) {
		if input.num_rows_one <= 1000 {
			let _ = logic::part_one (& input);
		}
		if input.num_rows_one <= 1_000_000 {
			let _ = logic::part_two (& input);
		}
	}
});
