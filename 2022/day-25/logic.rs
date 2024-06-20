use super::*;

use input::Input;
use model::Snafu;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <String> {
	let sum: Val =
		input.snafus.iter ()
			.map (Snafu::to_val)
			.try_fold (0, |sum, val| GenOk (chk! (sum + val ?) ?)) ?;
	let snafu = Snafu::from_val (sum);
	Ok (snafu.to_string ())
}
