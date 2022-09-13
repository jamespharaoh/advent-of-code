//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Dir;
use model::Portal;
use model::Pos;
use model::Tile::{ Empty, Letter, Passage, Wall };

type Paths = HashMap <Portal, Vec <(Portal, u32)>>;

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result (input, false)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	calc_result (input, true)
}

fn calc_result (input: & Input, recursive: bool) -> GenResult <u32> {
	let paths = get_paths (input) ?;
	let mut search = PrioritySearch::with_hash_map (
		|(portal, level), dist, mut adder: PrioritySearchAdder <_, _, _>| {
			if portal != Portal::ZZ.partner () {
				for & (next_portal, next_dist) in & paths [& portal] {
					let next_level =
						if next_portal.inner () { level + 1_i32 } else { level - 1_i32 };
					if next_portal == Portal::AA { continue }
					if recursive && (next_portal == Portal::ZZ) != (next_level < 0_i32) { continue }
					adder.add ((next_portal.partner (), next_level), dist + next_dist);
				}
			}
			(portal, level, dist)
		});
	search.push ((Portal::AA, 0_i32), 0);
	Ok (
		search
			.take_while (|& (_, _, dist)| dist <= 10_000)
			.find (|& (portal, level, _)|
				portal == Portal::ZZ.partner () && (! recursive || level == -1_i32))
			.map (|(_, _, dist)| dist - 1)
			.ok_or ("No solution found") ?
	)
}

fn get_paths (input: & Input) -> GenResult <Paths> {
	let portals = find_portals (input) ?;
	let portals_by_pos: HashMap <Pos, Portal> =
		portals.iter ()
			.map (|& (portal, pos_0, _)| (pos_0, portal))
			.collect ();
	let mut all_paths = Paths::new ();
	for (portal, pos_0, pos_1) in portals {
		let mut paths = Vec::new ();
		let mut seen = HashSet::new ();
		seen.insert (pos_0);
		seen.insert (pos_1);
		let mut todo = Vec::new ();
		todo.push ((pos_1, 0));
		while let Some ((pos, dist)) = todo.pop () {
			for adj_pos in pos.adjacent_4 () {
				if ! seen.insert (adj_pos) { continue }
				match input.grid.get (adj_pos) {
					Some (Passage) => todo.push ((adj_pos, dist + 1)),
					Some (Letter (_)) => {
						let next_portal =
							portals_by_pos.get (& adj_pos).copied ()
								.ok_or ("Invalid map") ?;
						paths.push ((next_portal, dist + 1));
					},
					Some (Wall) => (),
					Some (Empty) | None => return Err ("Invalid map".into ()),
				}
			}
		}
		if all_paths.insert (portal, paths).is_some () {
			return Err ("Duplicated portal".into ());
		}
	}
	if ! all_paths.contains_key (& Portal::AA) { return Err ("No portal AA".into ()) }
	if ! all_paths.contains_key (& Portal::ZZ) { return Err ("No portal ZZ".into ()) }
	for & portal in all_paths.keys () {
		if portal == Portal::AA || portal == Portal::ZZ { continue }
		if ! all_paths.contains_key (& portal.partner ()) {
			return Err ("Invalid map".into ());
		}
	}
	Ok (all_paths)
}

fn find_portals (input: & Input) -> GenResult <Vec <(Portal, Pos, Pos)>> {
	let mut portals = Vec::new ();
	for (pos, tile) in input.grid.iter () {
		if ! matches! (tile, Letter (_)) { continue }
		for dir in [ Dir::Right, Dir::Down ] {
			let (label, inner, pos_0, pos_1) = match (
				input.grid.get ((pos + (dir, -1)) ?),
				input.grid.get ((pos + (dir, 0)) ?),
				input.grid.get ((pos + (dir, 1)) ?),
				input.grid.get ((pos + (dir, 2)) ?),
			) {
				(None, Some (Letter (ch_0)), Some (Letter (ch_1)), Some (Passage)) =>
					([ ch_0, ch_1 ], false, (pos + (dir, 1)) ?, (pos + (dir, 2)) ?),
				(Some (Passage), Some (Letter (ch_0)), Some (Letter (ch_1)), Some (Empty)) =>
					([ ch_0, ch_1 ], true, pos, (pos + (dir, -1)) ?),
				(Some (Empty), Some (Letter (ch_0)), Some (Letter (ch_1)), Some (Passage)) =>
					([ ch_0, ch_1 ], true, (pos + (dir, 1)) ?, (pos + (dir, 2)) ?),
				(Some (Passage), Some (Letter (ch_0)), Some (Letter (ch_1)), None) =>
					([ ch_0, ch_1 ], false, pos, (pos + (dir, -1)) ?),
				_ => continue,
			};
			portals.push ((Portal::new (label, inner), pos_0, pos_1));
		}
	}
	Ok (portals)
}
