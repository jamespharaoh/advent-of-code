#![ no_main ]

use aoc_2016::day_24::*;
use libfuzzer_sys::fuzz_target;

fuzz_target! (|input_str: & str| {
	let mut input_vec: Vec <& str> = input_str.split ('\n').collect ();
	if let Some (width_str) = input_vec [0].strip_prefix ("FUZZ_WIDTH_FILTER=") {
		if let Ok (width) = width_str.parse::<usize> () {
			input_vec = input_vec.iter ().copied ()
				.skip (1)
				.filter (|line| width <= line.chars ().count ())
				.collect ();
			if input_vec.is_empty () { input_vec.push (""); }
		}
	}
	if let Some (width_str) = input_vec [0].strip_prefix ("FUZZ_WIDTH_TRUNCATE=") {
		if let Ok (width) = width_str.parse () {
			input_vec.remove (0);
			for line in input_vec.iter_mut () {
				if line.chars ().count () > width {
					let num_bytes = line.chars ()
						.take (width)
						.map (|ch| ch.len_utf8 ())
						.sum ();
					* line = & line [0 .. num_bytes];
				}
			}
			if input_vec.is_empty () { input_vec.push (""); }
		}
	}
	if let Ok (input) = model::Input::parse (& input_vec) {
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});
