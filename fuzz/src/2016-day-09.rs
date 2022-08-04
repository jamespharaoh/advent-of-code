#![ no_main ]

use aoc_2016::day_09::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	let _ = logic::part_one (input_str);
	let _ = logic::part_two (input_str);
});
