use super::*;
use input::Input;
use model::ProgInfo;

pub fn part_one (input: & Input) -> GenResult <String> {
	let root = ProgInfo::build (input) ?;
	Ok (root.name.to_owned ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let root = ProgInfo::build (input) ?;
	let mut cur = & root;
	let mut target_weight = root.total_weight;
	loop {
		let (min_weight, max_weight) =
			cur.holds.iter ()
				.map (|held| held.total_weight)
				.minmax ()
				.into_option ()
				.ok_or ("No solution found") ?;
		if min_weight == max_weight {
			return Ok (u32::sub_2 (
				u32::add_2 (cur.prog_weight, target_weight) ?,
				cur.total_weight,
			) ?);
		}
		let num_min =
			cur.holds.iter ()
				.filter (|held| held.total_weight == min_weight)
				.count ();
		let num_max =
			cur.holds.iter ()
				.filter (|held| held.total_weight == max_weight)
				.count ();
		if num_min + num_max != cur.holds.len () {
			return Err ("No solution found".into ());
		}
		let (wrong_weight, right_weight) =
			match (num_min, num_max) {
				(1, _) if num_max > 1 => (min_weight, max_weight),
				(_, 1) if num_min > 1 => (max_weight, min_weight),
				_ => return Err ("No solution found".into ()),
			};
		cur =
			cur.holds.iter ()
				.find (|held| held.total_weight == wrong_weight)
				.unwrap ();
		target_weight = right_weight;
	}
}
