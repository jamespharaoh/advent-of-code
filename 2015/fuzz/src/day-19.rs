#![ no_main ]

use aoc_2015::day_19::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	let input_vec = input_str.split ('\n').collect::<Vec <_>> ();
	if input_vec.last ().unwrap ().len () > 16 { return }
	if let Ok (input) = model::Input::parse (& input_vec) {
		let _ = logic::part_one (input.clone ());
		let _ = logic::part_two (input.clone ());
	}
});
