#![ no_main ]

use aoc_2015::day_04::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input: & str| {
	let input_vec =
		[ "NUM_ZEROS=1", "MAX_THREADS=1" ].iter ().copied ()
			.chain (input.trim ().split ('\n'))
			.collect::<Vec <& str>> ();
	if let Ok (input) = model::Input::parse (& input_vec) {
		let _ = logic::part_one (& input);
	}
	let input_vec =
		[ "NUM_ZEROS=2", "MAX_THREADS=1" ].iter ().copied ()
			.chain (input.trim ().split ('\n'))
			.collect::<Vec <& str>> ();
	if let Ok (input) = model::Input::parse (& input_vec) {
		let _ = logic::part_two (& input);
	}
});
