use super::*;

use input::Input;
use model::Coord;
use model::Dir;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let hq_pos = iter_posns (input)
		.take (1_000)
		.fold (Ok (Pos::ZERO), Result::and) ?;
	Ok (hq_pos.n.unsigned_abs () + hq_pos.e.unsigned_abs ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let hq_pos = iter_posns (input)
		.take (1_000)
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
