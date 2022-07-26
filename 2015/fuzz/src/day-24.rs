#![ no_main ]

use aoc_2015::day_24::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	let input_lines: Vec <& str> = input_str.trim ().split ('\n').collect ();
	if let Ok (input) = model::Input::parse (& input_lines) {
		let _ = logic::part_one (input.clone ());
		let _ = logic::part_two (input);
	}
});
