use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.digits.iter ()
			.circular_tuple_windows::<(_, _)> ()
			.filter (|& (& digit_0, & digit_1)| digit_0 == digit_1)
			.map (|(& digit, _)| digit.pan_u32 ())
			.sum ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	if input.digits.len () % 2 != 0 {
		return Err ("Must have an even number of digits".into ());
	}
	Ok (
		input.digits.iter ().take (input.digits.len () / 2)
			.zip (input.digits.iter ().skip (input.digits.len () / 2))
			.filter (|& (& digit_0, & digit_1)| digit_0 == digit_1)
			.map (|(& digit, _)| digit.pan_u32 () * 2)
			.sum ()
	)
}
