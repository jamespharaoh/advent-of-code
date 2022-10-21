//! Logic for solving the puzzles

use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let pub_key_0 = input.pub_key_0.pan_u64 ();
	let pub_key_1 = input.pub_key_1.pan_u64 ();
	let mut val = 1_u64;
	let mut num_loops = 0_u32;
	let pub_key = loop {
		if val == pub_key_0 { break pub_key_1 }
		if val == pub_key_1 { break pub_key_0 }
		val = chk! ((val * 7) % 20_201_227) ?;
		num_loops += 1;
		if num_loops == input.params.max_loops {
			return Err ("Giving up after max loops".into ());
		}
	};
	val = 1_u64;
	for _ in 0 .. num_loops {
		val = chk! ((val * pub_key) % 20_201_227) ?;
	}
	Ok (val.pan_u32 ())
}
