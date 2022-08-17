//! Logic for solving the puzzles

use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <i32> {
	let mut total: i32 = 0;
	for delta in input.deltas.iter_vals () {
		total += delta;
	}
	Ok (total)
}

pub fn part_two (input: & Input) -> GenResult <i32> {
	let mut total: i32 = 0;
	let mut seen: HashSet <i32> = HashSet::new ();
	for (idx, delta) in iter::repeat (input.deltas.clone ()).flatten ().enumerate () {
		if ! seen.insert (total) { break }
		if idx == 200_000 { return Err ("Giving up after 200k iterations".into ()) }
		total = i32::add_2 (total, delta) ?;
	}
	Ok (total)
}
