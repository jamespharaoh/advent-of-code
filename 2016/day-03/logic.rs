use super::*;

use input::Input;
use model::Side;

pub fn part_one (input: & Input) -> GenResult <usize> {
	sanity_check (input) ?;
	let num_possible =
		input.triangles.iter ()
			.map (|& (a, b, c)| [a, b, c])
			.map (|mut sides| { sides.sort (); sides })
			.filter (|& [a, b, c]| c < a + b)
			.count ();
	Ok (num_possible)
}

pub fn part_two (input: & Input) -> GenResult <usize> {
	sanity_check (input) ?;
	let num_possible =
		input.triangles.iter ()
			.tuples::<(_, _, _)> ()
			.flat_map (
				|(& (a0, a1, a2), & (b0, b1, b2), & (c0, c1, c2))|
					[(a0, b0, c0), (a1, b1, c1), (a2, b2, c2) ])
			.map (|(a, b, c)| [a, b, c])
			.map (|mut sides| { sides.sort (); sides })
			.filter (|& [a, b, c]| c < a + b)
			.count ();
	Ok (num_possible)
}

fn sanity_check (input: & Input) -> GenResult <()> {
	if input.triangles.iter ()
			.any (|& (a, b, c)| Side::add_3 (a, b, c).is_err ()) {
		return Err ("Overflow".into ());
	}
	Ok (())
}
