//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Pos;
use model::Step;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <Val> {
	sanity_check (input) ?;
	let map_0 = line_points (& input.wire_0) ?;
	let map_1 = line_points (& input.wire_1) ?;
	let set_0: HashSet <Pos> = map_0.keys ().copied ().collect ();
	let set_1: HashSet <Pos> = map_1.keys ().copied ().collect ();
	Ok (
		HashSet::intersection (& set_0, & set_1)
			.map (|pos| pos.x.abs () + pos.y.abs ())
			.min ()
			.ok_or ("No solution found") ?
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	sanity_check (input) ?;
	let map_0 = line_points (& input.wire_0) ?;
	let map_1 = line_points (& input.wire_1) ?;
	let set_0: HashSet <Pos> = map_0.keys ().copied ().collect ();
	let set_1: HashSet <Pos> = map_1.keys ().copied ().collect ();
	Ok (
		HashSet::intersection (& set_0, & set_1)
			.map (|pos| map_0 [pos] + map_1 [pos])
			.min ()
			.ok_or ("No solution found") ?
	)
}

fn sanity_check (input: & Input) -> GenResult <()> {
	for step in iter::empty ()
			.chain (& input.wire_0)
			.chain (& input.wire_1) {
		if step.num < 1 { return Err ("Step distance must be one or more".into ()) }
	}
	Ok (())
}

fn line_points (steps: & [Step]) -> GenResult <HashMap <Pos, u32>> {
	let mut result = HashMap::new ();
	let mut pos = Pos::ZERO;
	let mut dist = 0_u32;
	for step in steps {
		for _ in 0 .. step.num {
			pos = pos.try_add ((step.dir, 1)) ?;
			dist += 1;
			result.entry (pos).or_insert (dist);
		}
	}
	Ok (result)
}
