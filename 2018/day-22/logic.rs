//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Coord;
use model::Equip;
use model::Grid;
use model::Pos;
use model::Region;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <u32> {
	sanity_check (input) ?;
	let size = input.target + Pos { y: 1, x: 1 };
	let grid = calc_grid (input, size);
	let risk_level = grid.values ()
		.map (|region| match region {
			Region::Rocky => 0,
			Region::Wet => 1,
			Region::Narrow => 2,
		})
		.sum ();
	Ok (risk_level)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	sanity_check (input) ?;
	let base_dim = cmp::max (input.target.y, input.target.x) + 1;
	let base_size = Pos { y: base_dim, x: base_dim };
	'OUTER: for mul in 2 .. {
		let size = base_size * mul;
		let grid = calc_grid (input, size);
		let mut search = search::PrioritySearch::with_hash_map (|
			(pos, equip): (Pos, Equip),
			dist: u32,
			mut adder: search::PrioritySearchAdder <_, _, _>,
		| {
			for next_equip in [ Equip::Torch, Equip::Climbing, Equip::Neither ].iter ().copied () {
				if next_equip == equip { continue }
				if ! grid.get (pos).unwrap ().can_equip (next_equip) { continue }
				adder.add ((pos, next_equip), dist + 7);
			}
			for adj_pos in pos.adjacent_4 () {
				if adj_pos.x < 0 || adj_pos.y < 0 { continue }
				let adj_region = some_or! (grid.get (adj_pos), return None);
				if ! adj_region.can_equip (equip) { continue }
				adder.add ((adj_pos, equip), dist + 1);
			}
			Some ((pos, equip, dist))
		});
		search.push ((Pos::ZERO, Equip::Torch), 0);
		for item in search {
			if let Some ((pos, equip, dist)) = item {
				if dist > input.params.max_mins {
					return Err ("Giving up after 2k minutes".into ());
				}
				if pos == input.target && equip == Equip::Torch { return Ok (dist) }
			} else {
				continue 'OUTER;
			}
		}
		return Err ("No solution found".into ());
	}
	unreachable! ();
}

fn sanity_check (input: & Input) -> GenResult <()> {
	if input.target.x < 0 || input.target.y < 0 {
		return Err ("Target must have positive coordinates".into ());
	}
	if input.target.y >= input.params.max_target || input.target.x >= input.params.max_target {
		return Err ("Refusing to handle target more than 1000 distant in any axis".into ());
	}
	Ok (())
}

fn calc_grid (input: & Input, size: Pos) -> Grid {
	let mut grid_data = Vec::new ();
	let mut ero_row: Vec <Val> =
		itertools::iterate (input.depth, |prev| (prev + input.params.top_factor) % input.params.modulo)
			.take (size.x.as_usize ())
			.collect ();
	let mut y: Coord = 0;
	loop {
		grid_data.extend (ero_row.iter ().map (|& ero| match ero % 3 {
			0 => Region::Rocky,
			1 => Region::Wet,
			2 => Region::Narrow,
			_ => unreachable! (),
		}));
		y += 1;
		if y == size.y { break }
		let first = (y.as_u32 () * input.params.left_factor + input.depth) % input.params.modulo;
		ero_row = iter::empty ()
			.chain (iter::once (first))
			.chain (ero_row.iter ().skip (1).copied ()
				.enumerate ()
				.map (|(idx, val)| (Coord::from_usize (idx).unwrap () + 1, val))
				.scan (first, |state, (x, up)| {
					let left = * state;
					if (Pos { y, x }) == input.target {
						* state = input.depth % input.params.modulo;
					} else {
						* state = (left * up + input.depth) % input.params.modulo;
					}
					Some (* state)
				}))
			.collect ();
	}
	let grid_size = [ size.y.as_usize (), size.x.as_usize () ];
	Grid::wrap (grid_data, [0, 0], grid_size)
}
