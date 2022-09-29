use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.report.iter ().copied ()
			.tuple_windows ()
			.filter (|& (a, b)| a < b)
			.count ()
			.pan_u32 ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		input.report.iter ().copied ()
			.tuple_windows ()
			.filter (|& (a, _, _, b)| a < b)
			.count ()
			.pan_u32 ()
	)
}
