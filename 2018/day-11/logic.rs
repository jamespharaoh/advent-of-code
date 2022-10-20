//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Coord;
use model::Grid;
use model::Pos;
use model::Power;

pub fn part_one (input: & Input) -> GenResult <String> {
	let grid = gen_grid (input) ?;
	let (pos, _) = find_simple (& grid, 3) ?;
	Ok (format! ("{},{}", pos.x, pos.y))
}

pub fn part_two (input: & Input) -> GenResult <String> {
	let base_grid = gen_grid (input) ?;
	let base_grid_ref = & base_grid;
	let mut horiz_grid = base_grid.clone ();
	let mut vert_grid = base_grid.clone ();
	let mut small_grid = base_grid.clone ();
	let mut size = 1;
	let mut best = None;
	loop {
		for (pos, power) in small_grid.iter () {
			if let Some ((_, _, best_power)) = best {
				if best_power < power {
					best = Some ((pos, size, power));
				}
			} else {
				best = Some ((pos, size, power));
			}
		}
		if size == input.params.grid_size { break }
		size += 1;
		let grid_size = input.params.grid_size + 1 - size;
		let horiz_grid_ref = & horiz_grid;
		horiz_grid = Grid::wrap_range (
			(size ..= input.params.grid_size)
				.flat_map (|y| (1 ..= grid_size)
					.map (move |x| horiz_grid_ref.get (Pos { y, x }).unwrap ()
						+ base_grid_ref.get (Pos { y, x: x + size - 1 }).unwrap ()))
				.collect (),
			Pos::new (size, 1),
			Pos::new (input.params.grid_size + 1, grid_size + 1)) ?;
		let vert_grid_ref = & vert_grid;
		vert_grid = Grid::wrap_range (
			(1 ..= grid_size)
				.flat_map (|y| (size ..= input.params.grid_size)
					.map (move |x| vert_grid_ref.get (Pos { y, x }).unwrap ()
						+ base_grid_ref.get (Pos { y: y + size - 1, x }).unwrap ()))
				.collect (),
			Pos::new (1, size),
			Pos::new (grid_size + 1, input.params.grid_size + 1)) ?;
		let horiz_grid_ref = & horiz_grid;
		let vert_grid_ref = & vert_grid;
		let small_grid_ref = & small_grid;
		small_grid = Grid::wrap_range (
			(1 ..= grid_size)
				.flat_map (|y| (1 ..= grid_size)
					.map (move |x| small_grid_ref.get (Pos { y, x }).unwrap ()
						+ horiz_grid_ref.get (Pos { y: y + size - 1, x }).unwrap ()
						+ vert_grid_ref.get (Pos { y, x: x + size - 1 }).unwrap ()
						- base_grid_ref.get (Pos { y: y + size - 1, x: x + size - 1 }).unwrap ()))
				.collect (),
			Pos::new (1, 1),
			Pos::new (grid_size + 1, grid_size + 1)) ?;
	}
	if let Some ((pos, size, _)) = best {
		Ok (format! ("{},{},{}", pos.x, pos.y, size))
	} else { Err ("No solution found".into ()) }
}

pub fn find_simple (grid: & Grid, size: Coord) -> GenResult <(Pos, Power)> {
	(grid.first_key ().x ..= grid.last_key ().x - size + 1)
		.flat_map (|y_0| (grid.first_key ().y ..= grid.last_key ().y - size + 1)
			.map (move |x_0| {
				let pos_0 = Pos { y: y_0, x: x_0 };
				let mut power = 0_i32;
				for y_1 in y_0 .. y_0 + size {
					for x_1 in x_0 .. x_0 + size {
						let pos_1 = Pos { y: y_1, x: x_1 };
						power += grid.get (pos_1).unwrap ();
					}
				}
				(pos_0, power)
			}))
		.max_by_key (|& (_, power)| power)
		.ok_or ("No solution found".into ())
}

fn gen_grid (input: & Input) -> GenResult <Grid> {
	if input.params.grid_size < 3 { return Err ("Grid must be at least 3×3".into ()) }
	let mut grid = Grid::new_range (
		Pos::new (1, 1),
		Pos::new (input.params.grid_size + 1, input.params.grid_size + 1)) ?;
	for y in 1 ..= input.params.grid_size {
		for x in 1 ..= input.params.grid_size {
			let pos = Pos { y, x };
			grid.set (pos, calc_power (input, pos).ok_or ("Overflow") ?);
		}
	}
	Ok (grid)
}

fn calc_power (input: & Input, pos: Pos) -> Option <Power> {
	if pos.x < 1 || pos.x > input.params.grid_size
			|| pos.y < 1 || pos.y > input.params.grid_size {
		return None;
	}
	let rack_id = pos.x.pan_i32 () + 10_i32;
	let power_level = chk! ((rack_id * pos.y.pan_i32 () + input.serial) * rack_id).ok () ?;
	Some ((power_level % 1000_i32) / 100_i32 - 5_i32)
}
