#![ no_main ]

use aoc_2015::day_10::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	let input_vec = input_str.split ('\n').collect::<Vec <& str>> ();
	if let Ok (mut input) = model::Input::parse (& input_vec) {
		if input.iters_one > 15 { input.iters_one = 15; }
		if input.iters_two > 20 { input.iters_two = 20; }
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
