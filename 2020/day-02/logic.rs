//! Logic for solving the puzzles

use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.pass_policies.iter ()
			.filter (|pass_policy|
				(pass_policy.num_0 ..= pass_policy.num_1).contains (
					& pass_policy.password.chars ()
						.filter (|& ch| ch == pass_policy.ch)
						.count ()
						.as_u32 ()))
			.count ()
			.as_u32 ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		input.pass_policies.iter ()
			.filter (|pass_policy|
				pass_policy.password.chars ().enumerate ()
					.filter (|& (ch_idx, ch)|
						((ch_idx + 1).as_u32 () == pass_policy.num_0
								|| (ch_idx + 1).as_u32 () == pass_policy.num_1)
							&& ch == pass_policy.ch)
					.count () == 1)
			.count ()
			.as_u32 ()
	)
}
