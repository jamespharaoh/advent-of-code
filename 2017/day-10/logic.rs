use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let data: Vec <u8> =
		input.data
			.split (',')
			.map (str::parse)
			.try_collect () ?;
	let string = knot::calc_sparse (data.iter_vals (), 1);
	Ok (string [0].as_u32 () * string [1].as_u32 ())
}

pub fn part_two (input: & Input) -> GenResult <String> {
	let hash = knot::calculate_rounds (input.data.as_bytes (), input.params.rounds_two);
	let result =
		hash.iter_vals ()
			.map (|byte| format! ("{:02x}", byte))
			.collect ();
	Ok (result)
}
