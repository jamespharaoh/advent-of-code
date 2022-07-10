use aoc_common::*;

puzzle! {
	name = "Passage Pathing";
	year = 2021;
	day = 12;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;
	use model::Caves;

	pub fn part_one (lines: & [& str]) -> GenResult <u64> {
		let caves = model::parse_input (lines) ?;
		Ok (calc_result (& caves, (), |_, route, next_cave| {
			if caves.is_small (next_cave) && route.contains (& next_cave) { return false }
			true
		}))
	}

	pub fn part_two (lines: & [& str]) -> GenResult <u64> {
		let caves = model::parse_input (lines) ?;
		let dupe_index: Option <(usize, usize)> = None;
		Ok (calc_result (& caves, dupe_index, |dupe_index, route, next_cave| {
			if let Some (dupe_index_val) = dupe_index {
				if dupe_index_val.1 + 1 > route.len ()
						|| route [dupe_index_val.0] != route [dupe_index_val.1] {
					* dupe_index = None;
				}
			}
			if caves.is_small (next_cave) {
				if next_cave == caves.start { return false }
				if let Some (prev_pos) = route.iter ()
						.position (|& some_cave| some_cave == next_cave) {
					if dupe_index.is_some () { return false }
					* dupe_index = Some ((prev_pos, route.len ()));
				}
			}
			true
		}))
	}

	fn calc_result <
		State,
		CheckFn: Fn (& mut State, & [usize], usize) -> bool,
	> (
		caves: & Caves,
		state: State,
		check_fn: CheckFn,
	) -> u64 {
		let mut state = state;
		let mut cache: HashMap <(usize, Vec <usize>), u64> = HashMap::new ();
		let mut route: Vec <usize> = vec! [ caves.start ];
		calc_recurse (caves, & mut state, & check_fn, & mut cache, & mut route)
	}

	fn calc_recurse <
		State,
		CheckFn: Fn (& mut State, & [usize], usize) -> bool,
	> (
		caves: & Caves,
		state: & mut State,
		check_fn: & CheckFn,
		cache: & mut HashMap <(usize, Vec <usize>), u64>,
		route: & mut Vec <usize>,
	) -> u64 {
		let this_cave = route.last ().unwrap ().to_owned ();
		if this_cave == caves.end { return 1 }
		let mut route_sorted = route.clone ();
		route_sorted.sort ();
		let cache_key = (this_cave, route_sorted);
		if let Some (& cached_val) = cache.get (& cache_key) { return cached_val }
		let mut num_routes: u64 = 0;
		for next_cave in caves.connexions [& this_cave].iter ().cloned () {
			if ! check_fn (state, & route, next_cave) { continue }
			route.push (next_cave);
			num_routes += calc_recurse (caves, state, check_fn, cache, route);
			route.pop ();
		}
		cache.insert (cache_key, num_routes);
		num_routes
	}

}

mod model {

	use super::*;

	pub fn parse_input (lines: & [& str]) -> GenResult <Caves> {
		let mut caves = Caves::new ();
		for line in lines {
			let line_parts: Vec <& str> = line.split ("-").collect ();
			if line_parts.len () != 2 { Err (format! ("Invalid input: {}", line)) ? }
			let cave_a = caves.name_idx (line_parts [0]);
			let cave_b = caves.name_idx (line_parts [1]);
			for (cave_0, cave_1) in [(cave_a, cave_b), (cave_b, cave_a)] {
				caves.connexions.entry (cave_0).or_insert (Vec::new ()).push (cave_1);
			}
		}
		Ok (caves)
	}

	pub struct Caves {
		pub connexions: HashMap <usize, Vec <usize>>,
		pub names: Vec <String>,
		pub start: usize,
		pub end: usize,
	}

	impl Caves {
		fn new () -> Caves {
			Caves {
				connexions: HashMap::new (),
				names: vec! [ "start".to_string (), "end".to_string () ],
				start: 0,
				end: 1,
			}
		}
		fn name_idx (& mut self, name: & str) -> usize {
			if let Some (pos) = self.names.iter ().position (|some_name| some_name == name) {
				return pos;
			}
			let pos = self.names.len ();
			self.names.push (name.to_string ());
			pos
		}
		pub fn is_small (& self, index: usize) -> bool {
			self.names [index].chars ().next ().unwrap ().is_lowercase ()
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_0: & [& str] = & [
		"start-A",
		"start-b",
		"A-c",
		"A-b",
		"b-d",
		"A-end",
		"b-end",
	];

	const EXAMPLE_1: & [& str] = & [
		"dc-end",
		"HN-start",
		"start-kj",
		"dc-start",
		"dc-HN",
		"LN-dc",
		"HN-end",
		"kj-sa",
		"kj-HN",
		"kj-dc",
	];

	const EXAMPLE_2: & [& str] = & [
		"fs-end",
		"he-DX",
		"fs-he",
		"start-DX",
		"pj-DX",
		"end-zg",
		"zg-sl",
		"zg-pj",
		"pj-he",
		"RW-he",
		"fs-DX",
		"pj-RW",
		"zg-RW",
		"start-pj",
		"he-WI",
		"zg-he",
		"pj-fs",
		"start-RW",
	];

	#[ test ]
	fn part_one_0 () -> GenResult <()> {
		assert_eq! (10, logic::part_one (EXAMPLE_0) ?);
		Ok (())
	}

	#[ test ]
	fn part_one_1 () -> GenResult <()> {
		assert_eq! (19, logic::part_one (EXAMPLE_1) ?);
		Ok (())
	}

	#[ test ]
	fn part_one_2 () -> GenResult <()> {
		assert_eq! (226, logic::part_one (EXAMPLE_2) ?);
		Ok (())
	}

	#[ test ]
	fn part_two_0 () -> GenResult <()> {
		assert_eq! (36, logic::part_two (EXAMPLE_0) ?);
		Ok (())
	}

	#[ test ]
	fn part_two_1 () -> GenResult <()> {
		assert_eq! (103, logic::part_two (EXAMPLE_1) ?);
		Ok (())
	}

	#[ test ]
	fn part_two_2 () -> GenResult <()> {
		assert_eq! (3509, logic::part_two (EXAMPLE_2) ?);
		Ok (())
	}

}
