#![ no_main ]

use aoc_2015::day_21::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	let input_lines: Vec <& str> = input_str.split ('\n').collect ();
	if let Ok (input) = model::Stats::parse (& input_lines) {
		let _ = logic::part_one (input.clone ());
		let _ = logic::part_two (input);
	}
});
