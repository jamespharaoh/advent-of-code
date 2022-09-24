//! Logic for solving the puzzles

use super::*;

use input::Input;
use input::InputParams;
use model::Grid;
use model::GridCursor;
use model::Pos;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u32> {

	let state = input.grid.clone ();

	let dirs = [
		Pos::new (-1, -1), Pos::new (-1, 0), Pos::new (-1, 1),
		Pos::new (0, -1), Pos::new (0, 1),
		Pos::new (1, -1), Pos::new (1, 0), Pos::new (1, 1),
	].map (|pos| state.offset (pos));

	calc_result (& input.params, state, |cursor| {
		let num_adj = dirs.iter ()
			.filter_map (|& dir| cursor.try_add (dir)
				.map (|adj_cursor| adj_cursor.item ())
				.ok ())
			.filter (|& tile| tile == Tile::Occupied)
			.count ();
		matches! ((cursor.item (), num_adj), (Tile::Empty, 0) | (Tile::Occupied, 0 ..= 3))
	})

}

pub fn part_two (input: & Input) -> GenResult <u32> {

	let state = input.grid.clone ();

	let dirs = [
		Pos::new (-1, -1), Pos::new (-1, 0), Pos::new (-1, 1),
		Pos::new (0, -1), Pos::new (0, 1),
		Pos::new (1, -1), Pos::new (1, 0), Pos::new (1, 1),
	].map (|pos| state.offset (pos));

	calc_result (& input.params, state, move |cursor| {
		let num_adj = dirs.iter ()
			.filter_map (|& dir| {
				let mut adj_cursor = cursor;
				loop {
					adj_cursor = adj_cursor.try_add (dir).ok () ?;
					let adj_tile = adj_cursor.item ();
					if adj_tile == Tile::Floor { continue }
					return Some (adj_tile);
				}
			})
			.filter (|& tile| tile == Tile::Occupied)
			.count ();
		matches! ((cursor.item (), num_adj), (Tile::Empty, 0) | (Tile::Occupied, 0 ..= 4))
	})

}

#[ inline ]
fn calc_result <EvalFn> (params: & InputParams, mut state: Grid, mut eval_fn: EvalFn) -> GenResult <u32>
		where EvalFn: FnMut (GridCursor) -> bool {

	for _ in 0 .. params.max_iters {

		let next_state = state.map (|cursor| {
			let tile = cursor.item ();
			if tile == Tile::Floor { return Tile::Floor }
			if eval_fn (cursor) { Tile::Occupied } else { Tile::Empty }
		});

		if state == next_state { 
			return Ok (
				state.values ()
					.filter (|& tile| tile == Tile::Occupied)
					.count ()
					.as_u32 ()
			);
		}

		state = next_state;

	}

	Err ("Giving up after max iterations".into ())

}
