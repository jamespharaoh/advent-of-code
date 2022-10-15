use super::*;

use input::Input;
use model::Pos;
use model::SeenGrid;
use model::Tile;
use search::PrioritySearch;
use search::PrioritySearchAdder;

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result (input, false)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	calc_result (input, true)
}

fn calc_result (input: & Input, round_trip: bool) -> GenResult <u32> {
	let (nums, dists) = calc_distances (input) ?;
	calc_shortest (& nums, & dists, round_trip)
}

/// Given the distance between each point of interest work out the shortest path between them.
///
/// This uses [`PrioritySearch`] to always check the shortest routes first. Along the way, it
/// tracks only the current position and the set of visited points so far, to allow efficient
/// short-circuiting.
///
fn calc_shortest (
	nums: & [(u8, Pos)],
	dists: & [(u8, u8, u32)],
	round_trip: bool,
) -> GenResult <u32> {

	let mut search = PrioritySearch::with_hash_map (
		|mut route: ArrayVec <u8, 11>, dist, mut adder: PrioritySearchAdder <_, _, _>| {
			let here = route.last ().copied ().unwrap ();
			route.sort ();
			for & (from, to, next_dist) in dists.iter () {
				if from != here { continue }
				if route.len () == nums.len () && to != 0 { continue }
				if route.len () < nums.len () && route.contains (& to) { continue }
				if route.len () < nums.len () + usize::from (round_trip) {
					let mut new_route = route.clone ();
					new_route.push (to);
					adder.add (new_route.clone (), dist + next_dist);
				}
			}
			(route, dist)
		});

	search
		.push (array_vec! [ 0_u8 ], 0)
		.filter (|& (ref route, _)| route.len () ==
			if round_trip { nums.len () + 1 } else { nums.len () })
		.map (|(_, dist)| dist)
		.next ()
		.ok_or_else (|| "No solution found".into ())

}

/// Work out the distance between every point of interest.
///
/// Points of interests include all of the number locations, ie the start point, all the points
/// that need to be visited and, in the case of part two, the end point.
///
fn calc_distances (
	input: & Input,
) -> GenResult <(Vec <(u8, Pos)>, Vec <(u8, u8, u32)>)> {

	// find list of numbers in grid

	let nums: Vec <(u8, Pos)> =
		input.tiles.iter ()
			.filter_map (|(pos, tile)| match tile {
				Tile::Num (num) => Some ((num, pos)),
				Tile::Wall | Tile::Open => None,
			})
			.sorted ()
			.collect ();

	// check for duplicates

	if nums.iter ().map (|& (num, _)| num).sorted ().dedup ().count () != nums.len () {
		return Err ("Duplicated nums".into ());
	}

	// check start exists

	if ! nums.iter ().any (|& (num, _)| num == 0_u8) {
		return Err ("No starting position".into ());
	}

	// assume we already visited all walls, reusing this makes things quicker

	let seen_template = SeenGrid::wrap_range (
		input.tiles.values ().map (|tile| matches! (tile, Tile::Wall)).collect (),
		input.tiles.start (),
		input.tiles.end ()) ?;

	// work out distances

	let mut dists: Vec <(u8, u8, u32)> = Vec::new ();
	'OUTER: for (start_idx, & (start_num, start_pos)) in nums.iter ().enumerate () {

		// track places visited so far

		let mut seen = seen_template.clone ();
		seen.set (start_pos, true);

		// track next places to visit

		let mut todo: VecDeque <(u32, Pos)> = VecDeque::new ();
		todo.push_back ((0, start_pos));

		// work out number of distance to find, allows some short-circuiting

		let mut num_to_find = nums.len () - 1 - start_idx;
		if num_to_find == 0 { continue }

		// iterate 'todo' places

		while let Some ((dist, pos)) = todo.pop_front () {

			// iterate adjacent positions

			for adj_pos in pos.adjacent_4 () {

				// track seen tiles and short-cirtcuit

				if seen.get (adj_pos).unwrap_or (true) { continue }
				seen.set (adj_pos, true);

				// add adjacent position to `todo`

				let adj_tile = input.tiles.get (adj_pos).unwrap ();
				todo.push_back ((dist + 1, adj_pos));

				// check if we reached a point-of-interest

				if let Tile::Num (num) = adj_tile {

					// only record it if it is greater than us, to prevent counting twice and
					// allow short-circuiting

					if num < start_num { continue }

					// record distance, both ways

					dists.push ((start_num, num, dist + 1));
					dists.push ((num, start_num, dist + 1));

					// abort path-finding if we found all the points greater than start

					num_to_find -= 1;
					if num_to_find == 0 { continue 'OUTER }

				}

			}

		}

		// due to short-circuiting, this loop should never complete if all the points are
		// connected

		return Err ("No solution found1".into ());

	}

	// return

	Ok ((nums, dists))

}
