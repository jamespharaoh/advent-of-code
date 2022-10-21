use super::*;

use input::Input;
use model::Coord;
use model::Dir;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let hq_pos = iter_posns (input)
		.fold (Ok (Pos::ZERO), Result::and) ?;
	Ok (hq_pos.n.unsigned_abs () + hq_pos.e.unsigned_abs ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let hq_pos = iter_posns (input)
		.scan (HashSet::new (), |seen, pos| {
			match pos {
				Ok (pos) => Some ((! seen.insert (pos)).then_some (Ok (pos))),
				Err (err) => Some (Some (Err (err))),
			}
		})
		.flatten ()
		.next ()
		.ok_or ("No solution found") ? ?;
	Ok (hq_pos.n.unsigned_abs () + hq_pos.e.unsigned_abs ())
}

fn check_input (input: & Input) -> GenResult <()> {
	if 1000 < input.steps.len () {
		return Err ("Max 1000 steps".into ());
	}
	if 100_000_i32 < input.steps.iter ().map (|& (_, dist)| dist).sum () {
		return Err ("Max distance 100k".into ());
	}
	Ok (())
}

pub fn iter_posns (input: & Input) -> impl Iterator <Item = NumResult <Pos>> + '_ {
	let mut dir = Dir::North;
	let mut pos = Pos::ZERO;
	let mut dist = Coord::ZERO;
	let mut steps_iter = input.steps.iter ();
	iter::from_fn (move || {
		if dist == Coord::ZERO {
			let & (turn, new_dist) = steps_iter.next () ?;
			dir = dir + turn;
			dist = new_dist;
		}
		true.then (|| {
			pos = (pos + (dir, Coord::ONE)) ?;
			dist -= Coord::ONE;
			Ok (pos)
		})
	})
}
