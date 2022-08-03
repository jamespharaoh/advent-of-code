#![ no_main ]

use aoc_2016::day_05::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	let input_vec = vec! [ "NUM_ZEROS=1", input_str ];
	if let Ok (input) = model::Input::parse (& input_vec) {
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
