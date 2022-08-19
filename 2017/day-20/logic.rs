//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Aspect;
use model::Axis;
use model::Coord;
use model::Particle;

pub fn part_one (input: & Input) -> GenResult <u32> {
	if input.particles.is_empty () { return Err ("No particles provided".into ()) }
	let (idx, _, _, _) =
		input.particles.iter ().copied ()
			.enumerate ()
			.map (|(idx, mut part)| {
				for axis in [ Axis::X, Axis::Y, Axis::Z ] {
					if [ Aspect::Acc, Aspect::Vel, Aspect::Pos ].iter ().copied ()
							.map (|aspect| part [aspect] [axis].signum ())
							.find (|& sign| sign != 0)
							.unwrap_or (0) < 0 {
						part.acc [axis] = - part.acc [axis];
						part.vel [axis] = - part.vel [axis];
						part.pos [axis] = - part.pos [axis];
					}
				}
				Ok::<_, nums::Overflow> ((
					idx,
					Coord::add_3 (part.acc.x, part.acc.y, part.acc.z) ?,
					Coord::add_3 (part.vel.x, part.vel.y, part.vel.z) ?,
					Coord::add_3 (part.pos.x, part.pos.y, part.pos.z) ?,
				))
			})
			.fold_ok (None, |min: Option <(usize, Coord, Coord, Coord)>, item| min
				.map (|min| {
					let min_key = (min.1, min.2, min.3);
					let item_key = (item.1, item.2, item.3);
					if min_key < item_key { min } else { item }
				}).or (Some (item))) ?
			.unwrap ();
	Ok (idx.as_u32 ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	if input.particles.is_empty () { return Ok (0) }
	if input.particles.len () == 1 { return Ok (1) }
	struct PartState { _idx: usize, part: Particle, alive: bool }
	let mut parts: Vec <PartState> =
		input.particles.iter ().copied ()
			.enumerate ()
			.map (|(idx, part)| PartState { _idx: idx, part, alive: true })
			.collect ();
	let mut time = 0_u32;
	while ! parts.is_empty () {
		parts.sort_by_key (|state| state.part.pos);
		'IDX_0: for idx_0 in 0 .. parts.len () - 1 {
			let pos_0 = parts [idx_0].part.pos;
			for idx_1 in idx_0 + 1 .. parts.len () {
				if ! parts [idx_1].alive { continue }
				let pos_1 = parts [idx_1].part.pos;
				if pos_0 != pos_1 { continue 'IDX_0 }
				parts [idx_0].alive = false;
				parts [idx_1].alive = false;
			}
		}
		if time == 50 { break } // TODO this is hacky (but quite fast)
		parts.retain (|state| state.alive);
		for state in parts.iter_mut () { state.part = next (state.part) ?; }
		time += 1;
	}
	Ok (parts.iter ().filter (|& state| state.alive).count ().as_u32 ())
}

fn next (mut part: Particle) -> NumResult <Particle> {
	part.vel.x = Coord::add_2 (part.vel.x, part.acc.x) ?;
	part.vel.y = Coord::add_2 (part.vel.y, part.acc.y) ?;
	part.vel.z = Coord::add_2 (part.vel.z, part.acc.z) ?;
	part.pos.x = Coord::add_2 (part.pos.x, part.vel.x) ?;
	part.pos.y = Coord::add_2 (part.pos.y, part.vel.y) ?;
	part.pos.z = Coord::add_2 (part.pos.z, part.vel.z) ?;
	Ok (part)
}
