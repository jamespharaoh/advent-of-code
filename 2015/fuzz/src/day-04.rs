#![ no_main ]

use aoc_2015::day_04::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input: & str| {
	let _ = logic::calc_result (input, 2);
});
