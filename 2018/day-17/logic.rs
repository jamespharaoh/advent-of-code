//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::ClayRange;
use model::Coord;
use model::Grid;
use model::Pos;
use model::Tile::{ self, DrySand, WetSand, Clay, Water };

pub fn part_one (input: & Input) -> GenResult <u32> {
	let grid = calc_grid (input) ?;
	Ok (count_tiles (& grid, |tile| matches! (tile, WetSand | Water)))
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let grid = calc_grid (input) ?;
	Ok (count_tiles (& grid, |tile| matches! (tile, Water)))
}

fn count_tiles (grid: & Grid, pred_fn: fn (Tile) -> bool) -> u32 {
	let mut num = 0_u32;
	for y in grid.first_key ().y ..= grid.last_key ().y {
		if num == 0 && ! (grid.first_key ().x ..= grid.last_key ().x)
				.any (|x| grid.get (Pos { y, x }).unwrap () == Clay) {
			continue;
		}
		for x in grid.first_key ().x ..= grid.last_key ().x {
			let pos = Pos { y, x };
			if pred_fn (grid.get (pos).unwrap ()) { num += 1; }
		}
	}
	num
}

fn calc_grid (input: & Input) -> GenResult <Grid> {

	if input.clay_ranges.is_empty () {
		return Err ("No clay ranges specified".into ());
	}

	if ! input.clay_ranges.iter ().copied ().all (ClayRange::valid) {
		return Err ("Invalid ranges specified".into ());
	}

	if input.clay_ranges.iter ().any (|clay_range|
		! (1 ..= 2000).contains (clay_range.y ().start ())
			|| ! (1 ..= 2000).contains (clay_range.y ().end ())
			|| ! (1 ..= 1000).contains (clay_range.x ().start ())
			|| ! (1 ..= 1000).contains (clay_range.x ().end ())) {
		return Err ("Clay ranges must be in range 1 to 1000 for x, 1 to 2000 for y".into ());
	}

	// create empty grid

	type Ranges = (RangeInclusive <Coord>, RangeInclusive <Coord>);

	let merge_ranges = |(left_y, left_x): Ranges, (right_y, right_x): Ranges| {
		let y_start = cmp::min (* left_y.start (), * right_y.start ());
		let y_end = cmp::max (* left_y.end (), * right_y.end ());
		let x_start = cmp::min (* left_x.start (), * right_x.start ());
		let x_end = cmp::max (* left_x.end (), * right_x.end ());
		(y_start ..= y_end, x_start ..= x_end)
	};

	let (clay_range_y, clay_range_x) =
		input.clay_ranges.iter ().copied ()
			.map (ClayRange::ranges)
			.reduce (merge_ranges)
			.unwrap ();

	let (grid_range_y, grid_range_x) =
		merge_ranges (
			(clay_range_y, clay_range_x.start () - 1 ..= clay_range_x.end () + 1),
			(0 ..= 0, 500 ..= 500));

	let grid_origin = Pos::new (
		- * grid_range_y.start (),
		- * grid_range_x.start ());

	let grid_size = Pos::new (
		Coord::from_usize (grid_range_y.len ()) ?,
		Coord::from_usize (grid_range_x.len ()) ?);

	let mut grid = Grid::new (grid_origin, grid_size);

	// place clay

	for range in input.clay_ranges.iter () {
		for y in range.y () {
			for x in range.x () {
				let pos = Pos { y, x };
				grid.set (pos, Clay);
			}
		}
	}

	// iterate over points where water is flowing

	let mut queue = Vec::with_capacity (grid_range_y.len () * 2);
	queue.push (Pos { y: 0, x: 500 });
	while let Some (pos) = queue.pop () {
		let tile_here = grid.get (pos).unwrap ();
		let tile_down = grid.get (pos.down (1) ?);
		match (tile_here, tile_down) {

			// retracing areas which are already handled

			(Water, Some (Water)) => (),
			(WetSand, Some (WetSand)) => (),

			// flowing down through dry sand

			(DrySand, Some (DrySand)) => {
				grid.set (pos, Tile::WetSand);
				queue.push (pos);
				queue.push (pos.down (1) ?);
			},

			// flowing down into an existing flow

			(DrySand, None | Some (WetSand)) => {
				grid.set (pos, Tile::WetSand);
			},

			// hitting a surface

			(DrySand | WetSand, Some (Clay | Water)) => {

				// check if we are enclosed or can flow off on each side

				fn check_dir (grid: & Grid, mut pos: Pos, next_fn: fn (Pos) -> Pos) -> (bool, Coord) {
					let bounded = loop {
						let next = next_fn (pos);
						match (grid.get (next).unwrap (), grid.get (next.down (1).unwrap ()).unwrap_or (DrySand)) {
							(DrySand | WetSand, Clay | Water) => pos = next,
							(Clay, _) => break true,
							(DrySand | WetSand, DrySand | WetSand) => break false,
							(Water, _) => unreachable! (),
						}
					};
					(bounded, pos.x)
				}

				let (left_bounded, left) = check_dir (& grid, pos, |pos| pos.left (1).unwrap ());
				let (right_bounded, right) = check_dir (& grid, pos, |pos| pos.right (1).unwrap ());

				if left_bounded && right_bounded {

					// fully enclosed, fill with water

					for x in left ..= right {
						grid.set (Pos { x, .. pos }, Water);
					}

				} else {

					// flowing off one or both sides, continue with those flows

					for x in left ..= right {
						grid.set (Pos { x, .. pos }, WetSand);
					}
					if ! left_bounded { queue.push (Pos { x: left, .. pos }.left (1).unwrap ()); }
					if ! right_bounded { queue.push (Pos { x: right, .. pos }.right (1).unwrap ()); }

				}
			},

			(WetSand, None | Some (DrySand)) | (Clay | Water, _) => unreachable! ("{tile_here:?} {tile_down:?}"),

		}
	}

	Ok (grid)

}
