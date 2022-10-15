use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.rows.iter ()
			.map (|row| {
				let max = row.iter ().max ().copied ().unwrap_or (0).pan_u32 ();
				let min = row.iter ().min ().copied ().unwrap_or (0).pan_u32 ();
				max - min
			})
			.sum ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	input.rows.iter ()
		.map (|row| row.iter ()
			.enumerate ()
			.flat_map (|(idx_0, & val_0)| row.iter ()
				.skip (idx_0 + 1)
				.filter_map (move |& val_1| {
					let val_low = cmp::min (val_0, val_1);
					let val_high = cmp::max (val_0, val_1);
					(val_high % val_low == 0).then_some (val_high / val_low)
				}))
			.exactly_one ()
			.map_err (|_err| GenError::from ("No solution found")))
		.try_fold (0_u32, |sum, item| {
			let item = item ?.pan_u32 ();
			Ok (chk! (sum + item) ?)
		})
}
