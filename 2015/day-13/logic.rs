use super::*;

use input::Input;
use model::HappinessTable;

pub fn part_one (input: & Input) -> GenResult <i32> {
	let table = HappinessTable::build (& input.pairs) ?;
	let mut perms_helper = PermutationsHelper::new_circular (table.len ());
	Ok (
		iter::from_fn (|| perms_helper.next ()
				.then (|| perms_helper.iter ()
					.circular_array_windows ()
					.map (|[& idx_0, & idx_1]| table [(idx_0, idx_1)] + table [(idx_1, idx_0)])
					.sum ()))
			.max ()
			.ok_or ("No solution found") ?
	)
}

pub fn part_two (input: & Input) -> GenResult <i32> {
	let table = HappinessTable::build (& input.pairs) ?;
	let mut perms_helper = PermutationsHelper::new_mirror (table.len ());
	Ok (
		iter::from_fn (|| perms_helper.next ()
				.then (|| perms_helper.iter ()
					.array_windows ()
					.map (|[& idx_0, & idx_1]| table [(idx_0, idx_1)] + table [(idx_1, idx_0)])
					.sum ()))
			.max ()
			.ok_or ("No solution found") ?
	)
}
