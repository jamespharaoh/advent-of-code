//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Coord;
use model::Pos;
use model::Region;

pub fn part_one (input: & Input) -> GenResult <u32> {
	sanity_check (input) ?;
	let strongest =
		input.nanobots.iter ()
			.max_by_key (|bot| bot.radius)
			.ok_or ("No solution found") ?;
	let num_in_range =
		input.nanobots.iter ()
			.map (|bot| posns_dist (bot.pos, strongest.pos))
			.filter (|dist| dist.as_ref ().map (|& dist| dist <= strongest.radius).unwrap_or (true))
			.try_fold (0_u32, |sum, item| { item ?; Ok::<_, Overflow> (sum + 1) }) ?;
	Ok (num_in_range)
}

pub fn part_two (input: & Input) -> GenResult <Coord> {
	sanity_check (input) ?;
	let regions: Vec <Region> =
		input.nanobots.iter ()
			.map (Region::from_bot)
			.try_collect () ?;
	let mut todo: Vec <(Region, usize, u32)> = Vec::new ();
	todo.push ((Region::OPEN, 0, 0));
	let mut best_num = 0_u32;
	let mut best_dist = Coord::MAX;
	let mut num_iters = 0;
	'OUTER: while let Some ((mut region, next_idx, mut num)) = todo.pop () {
		for next_idx in next_idx .. regions.len () {
			if num_iters == input.params.max_iters {
				return Err (format! (
					"Giving up after {max_iters} iterations.",
					max_iters = input.params.max_iters,
				).into ());
			}
			num_iters += 1;
			let num_remain =
				regions [next_idx .. ].iter ()
					.filter (|existing_region| existing_region.overlap (region).is_some ())
					.count ()
					.pan_u32 ();
			if num + num_remain < best_num { continue 'OUTER }
			let next_region = regions [next_idx];
			if let Some (overlap) = region.overlap (next_region) {
				if region != overlap {
					todo.push ((region, next_idx + 1, num));
				}
				region = overlap;
				num += 1;
			}
		}
		if best_num <= num {
			if let Some (dist) = region_dist (& region) ? {
				if best_num < num || dist < best_dist {
					best_num = num;
					best_dist = dist;
				}
			}
		}
	}
	if best_dist == Coord::MAX { return Err ("No solution found".into ()); }
	Ok (best_dist)
}

fn sanity_check (input: & Input) -> GenResult <()> {
	for bot in & * input.nanobots {
		if bot.radius < Coord::ZERO {
			return Err ("Radius must not be negative".into ());
		}
	}
	Ok (())
}

fn posns_dist (pos_0: Pos, pos_1: Pos) -> NumResult <Coord> {
	chk! (chk! (pos_0.x - pos_1.x) ?.abs (),
		+ chk! (pos_0.y - pos_1.y) ?.abs (),
		+ chk! (pos_0.z - pos_1.z) ?.abs ())
}

fn region_dist (region: & Region) -> NumResult <Option <Coord>> {
	let Region { ppp, pnn, npn, nnp } = some_or! (region.reduce () ?, return Ok (None));
	let a = [ ppp, pnn, npn, nnp ].into_iter ()
		.map (|rng| rng.dist ())
		.max ()
		.unwrap ();
	Ok (Some (a))
}
