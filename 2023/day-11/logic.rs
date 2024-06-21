use super::*;

use input::Input;
use model::Coord;
use model::Pos;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <Coord> {
	calc_result (input, input.params.expand_one)
}

pub fn part_two (input: & Input) -> GenResult <Coord> {
	calc_result (input, input.params.expand_two)
}

fn calc_result (input: & Input, expand: Coord) -> GenResult <Coord> {
	if input.grid.size ().x < 2 || input.grid.size ().y < 2 {
		return Err ("Grid size must be at least 2×2".into ());
	}
	let mut galaxy_posns: Vec <Pos> =
		input.grid.iter ()
			.filter (|& (_, tile)| tile == Tile::Galaxy)
			.map (|(pos, _)| pos)
			.collect ();
	if galaxy_posns.len () < 2 {
		return Err ("Grid must contain at least two galaxies".into ());
	}
	let filled_x: HashSet <Coord> =
		galaxy_posns.iter ()
			.map (|& pos| pos.x)
			.collect ();
	for x in (input.grid.start ().x .. input.grid.end ().x)
			.filter (|& x| ! filled_x.contains (& x))
			.sorted_by_key (|& x| cmp::Reverse (x)) {
		for pos in & mut galaxy_posns {
			if pos.x < x { continue }
			chk! (pos.x += expand) ?;
		}
	}
	let filled_y: HashSet <Coord> =
		galaxy_posns.iter ()
			.map (|& pos| pos.y)
			.collect ();
	for y in (input.grid.start ().y .. input.grid.end ().y)
			.filter (|& y| ! filled_y.contains (& y))
			.sorted_by_key (|& y| cmp::Reverse (y)) {
		for pos in & mut galaxy_posns {
			if pos.y < y { continue }
			chk! (pos.y += expand) ?;
		}
	}
	let mut dists = 0;
	for idx_0 in 0 .. galaxy_posns.len () - 1 {
		let pos_0 = galaxy_posns [idx_0];
		for idx_1 in idx_0 + 1 .. galaxy_posns.len () {
			let pos_1 = galaxy_posns [idx_1];
			dists += (pos_0.x - pos_1.x).abs ();
			dists += (pos_0.y - pos_1.y).abs ();
		}
	}
	Ok (dists)
}
