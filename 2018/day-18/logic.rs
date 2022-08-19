//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Coord;
use model::Grid;
use model::Tile::{ Open, Tree, Yard };

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result (input, 10)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	calc_result (input, 1_000_000_000)
}

fn calc_result (input: & Input, num_reps: u32) -> GenResult <u32> {

	if [ input.grid.last_key ().x, input.grid.last_key ().y ].contains (& Coord::MAX) {
		return Err ("Refusing to handle large grid".into ());
	}

	// iterate the specified number of times

	let mut grid = input.grid.clone ();
	let mut cache: HashMap <Grid, u32> = HashMap::new ();
	cache.insert (grid.clone (), 0);
	let mut cur_rep = 0_u32;
	while cur_rep < num_reps {

		// calculate the next state

		grid = Grid::wrap (
			grid.iter ()
				.map (|(pos, val)|
					(pos, val, [
						grid.get (pos.up (1).left (1)).unwrap_or (Open),
						grid.get (pos.up (1)).unwrap_or (Open),
						grid.get (pos.up (1).right (1)).unwrap_or (Open),
						grid.get (pos.left (1)).unwrap_or (Open),
						grid.get (pos.right (1)).unwrap_or (Open),
						grid.get (pos.down (1).left (1)).unwrap_or (Open),
						grid.get (pos.down (1)).unwrap_or (Open),
						grid.get (pos.down (1).right (1)).unwrap_or (Open),
					]))
				.map (|(_, here, around)| {
					let num_trees = around.iter ().filter (|&& tile| tile == Tree).count ();
					let num_yards = around.iter ().filter (|&& tile| tile == Yard).count ();
					match (here, num_trees, num_yards) {
						(Open, 3 .. , _) => Tree,
						(Open, _, _) => Open,
						(Tree, _, 3 .. ) => Yard,
						(Tree, _, _) => Tree,
						(Yard, 1 .. , 1 .. ) => Yard,
						(Yard, _, _) => Open,
					}
				})
				.collect (),
			grid.native_origin (),
			grid.native_size (),
		);
		cur_rep += 1;

		// detect loops and fast forward, otherwise add new state to cache

		if let Some (prev_rep) = cache.get (& grid) {
			let loop_reps = cur_rep - prev_rep;
			if num_reps < cur_rep + loop_reps { continue }
			let num_loops = (num_reps - cur_rep) / loop_reps;
			cur_rep += num_loops * loop_reps;
		} else {
			cache.insert (grid.clone (), cur_rep);
		}

	}

	// count trees and yards, and return their product

	let num_trees = grid.values ().filter (|& tile| tile == Tree).count ().as_u32 ();
	let num_yards = grid.values ().filter (|& tile| tile == Yard).count ().as_u32 ();

	Ok (num_trees * num_yards)

}
