use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.pairs.iter ()
			.filter (|pair| {
				let first_range = pair.first_start ..= pair.first_end;
				let second_range = pair.second_start ..= pair.second_end;
				(first_range.contains (& pair.second_start)
						&& first_range.contains (& pair.second_end))
					|| (second_range.contains (& pair.first_start)
						&& second_range.contains (& pair.first_end))
			})
			.count ()
			.pan_u32 ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		input.pairs.iter ()
			.filter (|pair|
				pair.first_start <= pair.second_end && pair.second_start <= pair.first_end)
			.count ()
			.pan_u32 ()
	)
}
