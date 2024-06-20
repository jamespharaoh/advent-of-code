use super::*;

use input::Input;
use input::Tile;
use model::Grid;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let (start, end) = calc_start_end (& input.grid) ?;
	Ok (calc_result (& input.grid, start, & [ end ]))
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let (start, end) = calc_start_end (& input.grid) ?;
	Ok (calc_result (& input.grid, start, & [ end, start, end ]))
}

fn calc_start_end (grid: & Grid) -> GenResult <(Pos, Pos)> {
	let (start, _) =
		grid.iter ()
			.filter (|& (pos, _)| pos.y == grid.end ().y - 1)
			.filter (|& (_, tile)| tile == Tile::Clear)
			.exactly_one ()
			.ok_or ("Error finding start position") ?;
	let (end, _) =
		grid.iter ()
			.filter (|& (pos, _)| pos.y == grid.start ().y)
			.filter (|& (_, tile)| tile == Tile::Clear)
			.exactly_one ()
			.ok_or ("Error finding end position") ?;
	Ok ((start, end))
}

fn calc_result (grid: & Grid, start: Pos, route: & [Pos]) -> u64 {
	let mut blizs: Vec <(Pos, Pos)> =
		grid.iter ()
			.filter_map (|(pos, tile)| match tile {
				Tile::BlizzardLeft => Some ((pos, Pos::new (-1, 0))),
				Tile::BlizzardRight => Some ((pos, Pos::new (1, 0))),
				Tile::BlizzardDown => Some ((pos, Pos::new (0, -1))),
				Tile::BlizzardUp => Some ((pos, Pos::new (0, 1))),
				Tile::Clear | Tile::Wall => None,
			})
			.collect ();
	let mut posns = HashSet::from ([ (start, 0) ]);
	let mut num_steps = 0;
	'OUTER: loop {
		blizs =
			blizs.into_iter ()
				.map (|(mut pos, dir)| {
					pos += dir;
					if pos.x == 0 { pos.x = grid.end ().x - 2; }
					if pos.x == grid.end ().x - 1 { pos.x = 1 }
					if pos.y == 0 { pos.y = grid.end ().y - 2; }
					if pos.y == grid.end ().y - 1 { pos.y = 1 }
					(pos, dir)
				})
				.collect ();
		let bliz_posns: HashSet <Pos> =
			blizs.iter ()
				.map (|& (pos, _)| pos)
				.collect ();
		let mut new_posns = HashSet::new ();
		for (pos, dest_idx) in posns {
			if dest_idx == route.len () { break 'OUTER }
			for new_pos in pos.adjacent_4 ().into_iter ().chain ([ pos ]) {
				if bliz_posns.contains (& new_pos) { continue }
				match grid.get (new_pos) {
					Some (Tile::Wall) => continue,
					None => continue,
					_ => if new_pos == route [dest_idx] {
						new_posns.insert ((new_pos, dest_idx + 1));
					} else {
						new_posns.insert ((new_pos, dest_idx));
					},
				}
			}
		}
		posns = new_posns;
		num_steps += 1;
	}
	num_steps
}
