use super::*;

use input::Input;
use model::Coord;
use model::Grid;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let grid = calc_grid (input) ?;
	Ok (calc_surface (& grid))
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let mut grid = calc_grid (input) ?;
	remove_pockets (& mut grid);
	Ok (calc_surface (& grid))
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.points.is_empty () {
		return Err ("Must supply at least one point".into ());
	}
	for & point in & input.points {
		if point.x < 0 || point.y < 0 || point.z < 0
				|| 49 <= point.x || 49 <= point.y || 49 <= point.z {
			return Err ("Coordinates must be between 0 and 49".into ());
		}
	}
	Ok (())
}

fn calc_grid (input: & Input) -> GenResult <Grid> {
	let grid_end = input.points.iter ()
		.fold (Pos::ZERO, |end, point| Pos {
			x: cmp::max (end.x, point.x + 2),
			y: cmp::max (end.y, point.y + 2),
			z: cmp::max (end.z, point.z + 2),
		});
	let mut grid = Grid::new_range (Pos::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::NEG_ONE), grid_end) ?;
	for & point in & input.points {
		grid.set (point, true);
	}
	Ok (grid)
}

fn remove_pockets (grid: & mut Grid) {
	let offsets: Vec <GridOffset <Pos, 3>> =
		Pos::ZERO.adjacent_six ().iter ()
			.map (|& pos| grid.offset (pos).unwrap ())
			.collect ();
	let mut seen = Grid::new_range (grid.start (), grid.end ()).unwrap ();
	let mut todo = vec! [ grid.cursor (Pos::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::NEG_ONE)).unwrap () ];
	while let Some (cur) = todo.pop () {
		if cur.get (& seen) { continue }
		seen.set (cur.pos (), true);
		for & offset in & offsets {
			let Ok (adj_cur) = chk! (cur + offset) else { continue };
			if adj_cur.get (& * grid) { continue }
			todo.push (adj_cur);
		}
	}
	let mut grid_data = seen.into_storage ();
	for val in & mut grid_data { * val = ! * val; }
	* grid = GridBuf::wrap_range (grid_data, grid.start (), grid.end ()).unwrap ();
}

fn calc_surface (grid: & Grid) -> u32 {
	grid.iter ()
		.filter (|& (_, val)| val)
		.map (|(pos, _)| 6 - pos.adjacent_six ().into_iter ()
			.filter (|& pos| grid.get (pos).unwrap_or (false))
			.count ())
		.sum::<usize> ()
		.pan_u32 ()
}
