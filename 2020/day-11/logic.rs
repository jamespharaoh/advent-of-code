//! Logic for solving the puzzles

use super::*;

use input::Input;
use input::InputParams;
use model::Tiles;
use model::TilesCursor;
use model::Pos;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u32> {

	let state = input.tiles.clone ();

	let dirs: Vec <_> = [
		Pos::new (-1, -1), Pos::new (-1, 0), Pos::new (-1, 1),
		Pos::new (0, -1), Pos::new (0, 1),
		Pos::new (1, -1), Pos::new (1, 0), Pos::new (1, 1),
	].into_iter ()
		.map (|pos| state.offset (pos))
		.try_collect () ?;

	calc_result (& input.params, state, |state: & Tiles, cursor| {
		let num_adj = dirs.iter ()
			.filter_map (|& dir| cursor.try_add (dir)
				.map (|adj_cursor| adj_cursor.get (state))
				.ok ())
			.filter (|& tile| tile == Tile::Occupied)
			.count ();
		matches! ((cursor.get (state), num_adj), (Tile::Empty, 0) | (Tile::Occupied, 0 ..= 3))
	})

}

pub fn part_two (input: & Input) -> GenResult <u32> {

	let state = input.tiles.clone ();

	let dirs: Vec <_> = [
		Pos::new (-1, -1), Pos::new (-1, 0), Pos::new (-1, 1),
		Pos::new (0, -1), Pos::new (0, 1),
		Pos::new (1, -1), Pos::new (1, 0), Pos::new (1, 1),
	].into_iter ().map (|pos| state.offset (pos)).try_collect () ?;

	calc_result (& input.params, state, move |state, cursor| {
		let num_adj = dirs.iter ()
			.filter_map (|& dir| {
				let mut adj_cursor = cursor;
				loop {
					adj_cursor = adj_cursor.try_add (dir).ok () ?;
					let adj_tile = adj_cursor.get (state);
					if adj_tile == Tile::Floor { continue }
					return Some (adj_tile);
				}
			})
			.filter (|& tile| tile == Tile::Occupied)
			.count ();
		matches! ((cursor.get (state), num_adj), (Tile::Empty, 0) | (Tile::Occupied, 0 ..= 4))
	})

}

#[ inline ]
fn calc_result <EvalFn> (params: & InputParams, mut state: Tiles, mut eval_fn: EvalFn) -> GenResult <u32>
	where EvalFn: FnMut (& Tiles, TilesCursor) -> bool {

	for _ in 0 .. params.max_iters {

		let next_state = state.map (|cursor| {
			let tile = cursor.get (& state);
			if tile == Tile::Floor { return Tile::Floor }
			if eval_fn (& state, cursor) { Tile::Occupied } else { Tile::Empty }
		});

		if state == next_state { 
			return Ok (
				state.values ()
					.filter (|& tile| tile == Tile::Occupied)
					.count ()
					.pan_u32 ()
			);
		}

		state = next_state;

	}

	Err ("Giving up after max iterations".into ())

}
