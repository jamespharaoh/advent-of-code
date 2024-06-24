use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <i64> {
	let mut sum = 0;
	for history in & input.histories {
		let inc =
			calc_diffs (history) ?
				.iter ().rev ()
				.map (|row| row.last ().copied ().unwrap ())
				.try_fold (0, |diff, prev| chk! (prev + diff)) ?;
		chk! (sum += inc) ?;
	}
	Ok (sum)
}

pub fn part_two (input: & Input) -> GenResult <i64> {
	let mut sum = 0;
	for history in & input.histories {
		let inc =
			calc_diffs (history) ?
				.iter ().rev ()
				.map (|row| row.first ().copied ().unwrap ())
				.try_fold (0, |diff, prev| chk! (prev - diff)) ?;
		chk! (sum += inc) ?;
	}
	Ok (sum)
}

fn calc_diffs (history: & [i64]) -> GenResult <Vec <Vec <i64>>> {
	let mut rows = Vec::new ();
	rows.push (history.to_vec ());
	loop {
		let row: Vec <i64> =
			rows.last ().unwrap ().iter ()
				.array_windows ()
				.map (|[& a, & b]| chk! (b - a))
				.try_collect () ?;
		if row.is_empty () {
			return Err ("Unable to find repeating pattern in history".into ());
		}
		let is_zeros = row.iter ().all (|& val| val == 0);
		rows.push (row);
		if is_zeros { return Ok (rows) }
	}
}
