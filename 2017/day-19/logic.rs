//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Dir;
use model::Pos;
use model::Tile;
use model::Turn;

pub fn part_one (input: & Input) -> GenResult <String> {
	Ok (
		route_iter (input) ?
			.take (30_000)
			.filter_map (|(_, tile)|
				if let Tile::Letter (asc) = tile { Some (asc.pan_char ()) } else { None })
			.collect ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		route_iter (input) ?
			.take (30_000)
			.count ()
			.pan_u32 ()
	)
}

pub fn route_iter (
	input: & Input,
) -> GenResult <impl Iterator <Item = (Pos, Tile)> + '_> {

	// find start position

	let mut pos =
		input.grid.iter ()
			.take_while (|& (pos, _)| pos.row == 0)
			.filter (|& (_, tile)| tile == Tile::Vert)
			.map (|(pos, _)| pos)
			.exactly_one ()
			.ok ()
			.ok_or ("Must have exactly one start position") ?;

	// iterate over path

	let mut dir = Dir::Down;
	Ok (iter::from_fn (move || {
		let tile = input.grid.get (pos).unwrap_or (Tile::Empty);

		// move straight if on a line or a letter

		if matches! (tile, Tile::Vert | Tile::Horiz | Tile::Letter (_)) {
			let last_pos = pos;
			let dir_one = (dir, 1);
			pos = chk! (pos + dir_one).ok () ?;
			return Some ((last_pos, tile));
		}

		// turn if on a corner

		for turn in [ Turn::Left, Turn::Right ].iter ().copied () {
			let next_dir = dir + turn;
			let next_dir_one = (next_dir, 1);
			let next_pos = ok_or! (chk! (pos + next_dir_one), continue);
			if matches! (input.grid.get (next_pos),
					Some (Tile::Vert | Tile::Horiz | Tile::Letter (_))) {
				dir = next_dir;
				let last_pos = pos;
				pos = next_pos;
				return Some ((last_pos, tile));
			}
		}

		None
	}))
}
