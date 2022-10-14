//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Components;
use model::Port;

pub fn part_one (input: & Input) -> GenResult <u32> {
	sanity_check (input) ?;
	Ok (
		bridges_iter (input)
			.map (|comps| strength (input, comps))
			.max ()
			.unwrap ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	sanity_check (input) ?;
	let longest =
		bridges_iter (input)
			.map (Components::count_ones)
			.max ()
			.unwrap ();
	Ok (
		bridges_iter (input)
			.filter (|comps| comps.count_ones () == longest)
			.map (|comps| strength (input, comps))
			.max ()
			.unwrap ()
	)
}

fn sanity_check (input: & Input) -> GenResult <()> {
	let mut comps = input.comps.clone ();
	if comps.len () > Components::BITS.pan_usize () {
		return Err ("Too many components".into ());
	}
	comps.sort ();
	if comps.iter ()
		.tuple_windows::<(_, _)> ()
		.any (|(left, right)| left == right
			|| (left.port_0 == right.port_1 && left.port_1 == right.port_0)) {
		return Err ("Duplicated components".into ());
	}
	Ok (())
}

fn bridges_iter (input: & Input) -> impl Iterator <Item = Components> + '_ {
	let mut todo: Vec <(Components, Port)> = Vec::new ();
	todo.push ((0, 0));
	let mut seen = HashSet::new ();
	iter::from_fn (move || {
		let (comps, port) = some_or! (todo.pop (), return None);
		for (next_idx, & next) in input.comps.iter ().enumerate () {
			if port != next.port_0 && port != next.port_1 { continue }
			let next_mask = 1 << next_idx;
			if comps & next_mask != 0 { continue }
			let new_comps = comps | next_mask;
			let new_port = if port == next.port_0 { next.port_1 } else { next.port_0 };
			if seen.insert ((new_comps, new_port)) {
				todo.push ((new_comps, new_port));
			}
		}
		Some (comps)
	})
}

fn strength (input: & Input, comps: Components) -> u32 {
	let mut total = 0;
	for (comp_idx, & comp) in input.comps.iter ().enumerate () {
		let comp_mask = 1 << comp_idx;
		if comps & comp_mask == 0 { continue }
		total += comp.port_0.pan_u32 () + comp.port_1.pan_u32 ();
	}
	total
}
