#![ no_main ]

use aoc_2016::day_13::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	let input_vec =
		[ "MAX_DIST=100", "COUNT_DIST=50" ].iter ().copied ()
			.chain (input_str.trim ().split ('\n'))
			.collect::<Vec <_>> ();
	if let Ok (input) = model::Input::parse (& input_vec) {
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
