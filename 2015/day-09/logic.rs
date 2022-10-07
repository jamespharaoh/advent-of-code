use super::*;

use input::Input;
use model::Dist;
use model::DistTable;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let dist_table = DistTable::build (input);
	Ok (iter_distances (& dist_table).min ().unwrap ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let dist_table = DistTable::build (input);
	Ok (iter_distances (& dist_table).max ().unwrap ())
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.dists.is_empty () { return Err ("Must provide at least one distance".into ()) }
	if input.dists.len () > 60 { Err ("Refusing to handle more than 60 distances") ?; }
	let num_places =
		input.dists.iter ()
			.flat_map (|& (ref from, ref to, _)| [from, to])
			.fold (HashSet::new (), |mut set, item| { set.insert (item); set })
			.into_iter ()
			.count ();
	if num_places > 10 {
		Err ("Refusing to handle more than 10 places") ?;
	}
	if input.dists.len () != num_places * (num_places - 1) / 2 {
		Err ("Wrong number of distances for given number of places") ?;
	}
	Ok (())
}

fn iter_distances <'out> (
	dist_table: & 'out DistTable <'out>,
) -> impl Iterator <Item = Dist> + 'out {
	let mut perms_helper = PermutationsHelper::new (dist_table.len ());
	iter::from_fn (move || {
		if ! perms_helper.next () { return None }
		Some (
			perms_helper.iter ()
				.tuple_windows::<(_, _)> ()
				.map (|(& idx_0, & idx_1)| dist_table [(idx_0, idx_1)])
				.sum::<u32> ()
		)
	})
}
