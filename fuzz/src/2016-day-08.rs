#![ no_main ]

use aoc_2016::day_08::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> =
		vec! [ "WIDTH=10", "HEIGHT=6" ].into_iter ()
			.chain (input_str.trim ().split ('\n'))
			.collect ();
	if let Ok (input) = model::Input::parse (& input_vec) {
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
