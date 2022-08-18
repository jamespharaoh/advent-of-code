#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_common::*;
use aoc_2018::day_14::*;
use input::Input;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.max_recipes = cmp::min (input.params.max_recipes, 100_000);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
