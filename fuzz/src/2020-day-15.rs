#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_2020::day_15::*;
use aoc_common::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.ord_one = cmp::min (input.params.ord_one, 10_000);
		input.params.ord_two = cmp::min (input.params.ord_two, 10_000);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
