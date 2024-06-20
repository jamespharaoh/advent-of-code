use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let mut prod = 1;
	for (& time, & record) in iter::zip (& input.times, & input.distances) {
		let mut sum = 0;
		for charge in 0 ..= time {
			let score = chk! (charge * (time - charge)) ?;
			if record < score {
				chk! (sum += 1) ?;
			}
		}
		chk! (prod *= sum) ?;
	}
	Ok (prod)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let time_strings: Vec <String> =
		input.times.iter ()
			.map (u32::to_string)
			.collect ();
	let time_string: String =
		time_strings.iter ()
			.flat_map (|val| val.chars ())
			.collect ();
	let record_strings: Vec <String> =
		input.distances.iter ()
			.map (u32::to_string)
			.collect ();
	let record_string: String =
		record_strings.iter ()
			.flat_map (|val| val.chars ())
			.collect ();
	let time: u64 = time_string.parse () ?;
	let record: u64 = record_string.parse () ?;
	let mut lose_0 = 0;
	let mut win_0 = time / 2;
	let mut win_1 = time / 2;
	let mut lose_1 = time;
	if calc_result (time, record, lose_0) ? || calc_result (time, record, lose_1) ? {
		return Err ("No losing time".into ());
	}
	if ! calc_result (time, record, win_0) ? {
		return Err ("No winning time".into ());
	}
	while 1 < win_0 - lose_0 {
		let charge = (win_0 + lose_0) / 2;
		if calc_result (time, record, charge) ? {
			win_0 = charge;
		} else {
			lose_0 = charge;
		}
	}
	while 1 < lose_1 - win_1 {
		let charge = (win_1 + lose_1) / 2;
		if calc_result (time, record, charge) ? {
			win_1 = charge;
		} else {
			lose_1 = charge;
		}
	}
	Ok (lose_1 - win_0)
}

fn calc_result (time: u64, record: u64, charge: u64) -> GenResult <bool> {
	Ok (record < chk! (charge * (time - charge)) ?)
}
