#![ no_main ]

use aoc_2015::day_20::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	if let Ok (input) = input_str.parse::<u32> () {
		if input < 1000000 {
			let _ = logic::part_one (input.clone ());
			let _ = logic::part_two (input.clone ());
		}
	}
});
