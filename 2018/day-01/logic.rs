//! Logic for solving the puzzles

use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <i32> {
	Ok (
		input.deltas.iter ()
			.sum::<i32> ()
	)
}

pub fn part_two (input: & Input) -> GenResult <i32> {
	if input.deltas.is_empty () { return Err ("No deltas provided".into ()) }
	let mut total: i32 = 0;
	let mut seen: HashSet <i32> = HashSet::new ();
	for (idx, delta) in iter::repeat (input.deltas.clone ()).flatten ().enumerate () {
		if ! seen.insert (total) { break }
		if idx == 200_000 { return Err ("Giving up after 200k iterations".into ()) }
		total = chk! (total + delta) ?;
	}
	Ok (total)
}
