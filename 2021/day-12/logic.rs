use super::*;

use input::Input;
use model::CaveId;
use model::Caves;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let caves = Caves::build (input) ?;
	Ok (calc_result (& caves, (), |_, route, next_cave| {
		if caves [next_cave].small && route.contains (& next_cave) { return false }
		true
	}))
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let caves = Caves::build (input) ?;
	let dupe_indexes: Option <(usize, usize)> = None;
	Ok (calc_result (& caves, dupe_indexes, |dupe_indexes, route, next_cave| {
		if let Some ((dupe_idx_0, dupe_idx_1)) = * dupe_indexes {
			if dupe_idx_1 + 1 > route.len ()
					|| route [dupe_idx_0] != route [dupe_idx_1] {
				* dupe_indexes = None;
			}
		}
		if caves [next_cave].small {
			if next_cave == caves.start ().id { return false }
			if let Some (prev_pos) = route.iter ()
					.position (|& some_cave| some_cave == next_cave) {
				if dupe_indexes.is_some () { return false }
				* dupe_indexes = Some ((prev_pos, route.len ()));
			}
		}
		true
	}))
}

fn calc_result <
	State,
	CheckFn: Fn (& mut State, & [CaveId], CaveId) -> bool,
> (
	caves: & Caves,
	state: State,
	check_fn: CheckFn,
) -> u64 {
	let mut state = state;
	let mut cache: HashMap <CacheKey, u64> = HashMap::new ();
	let mut route: Vec <CaveId> = vec! [ caves.start ().id ];
	calc_recurse (caves, & mut state, & check_fn, & mut cache, & mut route)
}

fn calc_recurse <
	State,
	CheckFn: Fn (& mut State, & [CaveId], CaveId) -> bool,
> (
	caves: & Caves,
	state: & mut State,
	check_fn: & CheckFn,
	cache: & mut HashMap <CacheKey, u64>,
	route: & mut Vec <CaveId>,
) -> u64 {
	let this_cave = route.last ().unwrap ().to_owned ();
	if this_cave == caves.end ().id { return 1 }
	let cache_key = make_cache_key (caves, route, this_cave);
	if let Some (& cached_val) = cache.get (& cache_key) { return cached_val }
	let mut num_routes: u64 = 0;
	for & next_cave in & caves [this_cave].cnxns {
		if ! check_fn (state, route, next_cave) { continue }
		route.push (next_cave);
		num_routes += calc_recurse (caves, state, check_fn, cache, route);
		route.pop ();
	}
	cache.insert (cache_key, num_routes);
	num_routes
}

type CacheKey = (u64, CaveId, CaveId);

fn make_cache_key (caves: & Caves, route: & [CaveId], this_cave: CaveId) -> CacheKey {
	let mut route_bits = 0_u64;
	let mut dupe = 0;
	for & cave in route {
		if ! caves [cave].small { continue }
		let bit = 1 << cave;
		if route_bits & bit == 0 {
			route_bits |= bit;
		} else {
			dupe = cave;
		}
	}
	(route_bits, dupe, this_cave)
}
