#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_common::*;
use aoc_2018::day_09::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.num_players = cmp::min (input.num_players, 1000);
		input.last_marble = cmp::min (input.last_marble, 10_000);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
