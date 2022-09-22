//! Logic for solving the puzzles

use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut time_now = input.earliest;
	let bus_ids: Vec <u32> =
		input.bus_ids.iter ().copied ()
			.flatten ()
			.collect ();
	for _ in 0 .. input.params.max_iters {
		for & bus_id in bus_ids.iter () {
			if chk! (time_now % bus_id) ? == 0 {
				return Ok (chk! (bus_id * (time_now - input.earliest)) ?);
			}
		}
		time_now += 1;
	}
	Err ("Giving up after max iterations".into ())
}

pub fn part_two (input: & Input) -> GenResult <u128> {
	let bus_idx_ids: Vec <(u128, u128)> =
		input.bus_ids.iter ().copied ().enumerate ()
			.filter_map (|(idx, id)| id.map (|id| (idx.as_u128 (), id.as_u128 ())))
			.collect ();
	let mut time: u128 = 0;
	for _ in 0 .. input.params.max_iters {
		let mut incr: u128 = 1;
		let mut matches = true;
		for & (bus_idx, bus_id) in & bus_idx_ids {
			if chk! ((time + bus_idx) % bus_id) ? == 0 {
				chk! (incr *= bus_id) ?;
			} else {
				matches = false;
			}
		}
		if matches { return Ok (time) }
		chk! (time += incr) ?;
	}
	Err ("Giving up after max iterations".into ())
}
