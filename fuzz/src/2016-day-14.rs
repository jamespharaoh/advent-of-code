#![ no_main ]

use aoc_2016::day_14::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	let input_vec =
		[ "NUM_KEYS=2", "NUM_NEXT=300", "HASH_REPS=4", "MAX_THREADS=1" ].iter ().copied ()
			.chain (input_str.trim ().split ('\n'))
			.collect::<Vec <_>> ();
	if let Ok (input) = model::Input::parse (& input_vec) {
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
