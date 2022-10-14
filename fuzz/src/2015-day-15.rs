#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_2015::day_15::*;
use aoc_common::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (input) = Input::parse_from_lines (& input_vec) {
		if input.ingrs.len () <= 4 {
			let _ = logic::part_one (& input);
			let _ = logic::part_two (& input);
		}
	}
});
