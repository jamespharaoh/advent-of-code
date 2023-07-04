use super::*;

use input::Disc;
use input::Input;

/// Implementation for part one
///
pub fn part_one (input: & Input) -> GenResult <u64> {
	calc_result (& input.discs)
}

/// Implementation for part two
///
pub fn part_two (input: & Input) -> GenResult <u64> {
	calc_result (
		& input.discs.iter ().copied ()
			.chain (iter::once (Disc {
				delay: input.discs.len ().pan_u8 () + 1,
				num_posns: 11,
				start_pos: 0,
			}))
			.collect::<Vec <Disc>> ()
	)
}

/// Main logic for both parts, takes a list of discs and calculates the start time.
///
/// We iterate over each disc, finding the first start time at which all discs up to this point
/// will be in the right positions. We start off with start time zero and add one each time,
/// but we also increase the stepping each time to the lowest common multiplier of all discs so
/// far.
/// 
pub fn calc_result (discs: & [Disc]) -> GenResult <u64> {
	let (time, _) = discs.iter ()
		.try_fold ((0_u64, 1_u64), |(time, step), disc|
			Ok::<_, GenError> ((
				(time .. )
					.step_by (step.pan_usize ())
					.take (disc.num_posns.pan_usize ())
					.find (|time|
						(time + disc.delay.pan_u64 () + disc.start_pos.pan_u64 ())
							% disc.num_posns.pan_u64 () == 0)
					.ok_or ("No solution found") ?,
				(step .. )
					.step_by (step.pan_usize ())
					.find (|step| step % disc.num_posns.pan_u64 () == 0)
					.unwrap (),
			))) ?;
	Ok (time)
}
