#![ no_main ]

use aoc_2016::day_25::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> =
			[ "LIMIT=400" ].iter ().copied ()
		.chain (input_str.split ('\n'))
		.collect ();
	if let Ok (input) = model::Input::parse (& input_vec) {
		let _ = logic::part_one (& input);
	}
});
