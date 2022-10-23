use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <usize> {
	check_input (input) ?;
	let num_possible =
		input.triangles.iter ()
			.map (|& (a, b, c)| [a, b, c])
			.map (|mut sides| { sides.sort (); sides })
			.filter (|& [a, b, c]| c < a + b)
			.count ();
	Ok (num_possible)
}

pub fn part_two (input: & Input) -> GenResult <usize> {
	check_input (input) ?;
	let num_possible =
		input.triangles.iter ()
			.arrays ()
			.flat_map (
				|[& (a0, a1, a2), & (b0, b1, b2), & (c0, c1, c2)]|
					[(a0, b0, c0), (a1, b1, c1), (a2, b2, c2) ])
			.map (|(a, b, c)| [a, b, c])
			.map (|mut sides| { sides.sort (); sides })
			.filter (|& [a, b, c]| c < a + b)
			.count ();
	Ok (num_possible)
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.triangles.iter ()
			.any (|& (a, b, c)| chk! (a + b + c).is_err ()) {
		return Err ("Overflow".into ());
	}
	Ok (())
}
