use super::*;
use input::Input;
use model::Pos;
use model::VHexDir;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let final_pos =
		posns_iter (input.steps.iter_vals ())
			.last ()
			.unwrap_or (Pos::ZERO);
	Ok (calc_dist (final_pos))
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		posns_iter (input.steps.iter_vals ())
			.map (calc_dist)
			.max ()
			.unwrap_or (0)
	)
}

fn posns_iter (
	mut steps_iter: impl Iterator <Item = VHexDir>,
) -> impl Iterator <Item = Pos> {
	use VHexDir::{ NorthWest, North, NorthEast, SouthWest, South, SouthEast };
	let mut pos = Pos::ZERO;
	iter::from_fn (move || {
		let step = steps_iter.next () ?;
		pos += match step {
			NorthWest => Pos { n: 1, e: -1 },
			North => Pos { n: 2, e: 0 },
			NorthEast => Pos { n: 1, e: 1 },
			SouthWest => Pos { n: -1, e: -1 },
			South => Pos { n: -2, e: 0 },
			SouthEast => Pos { n: -1, e: 1 },
		};
		Some (pos)
	})
}

fn calc_dist (pos: Pos) -> u32 {
	let horiz_steps = pos.e.abs ();
	let vert_steps = cmp::max (pos.n.abs () - horiz_steps, 0) / 2;
	horiz_steps.pan_u32 () + vert_steps.pan_u32 ()
}
