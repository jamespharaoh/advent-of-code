//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Coord;
use model::GenPos;
use model::Grid;
use model::GridOffset;
use model::PosXYZ;
use model::PosXYZW;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result::<PosXYZ, 3> (input, input.params.iters_one)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	calc_result::<PosXYZW, 4> (input, input.params.iters_two)
}

fn calc_result <Pos: GenPos <DIMS>, const DIMS: usize> (
	input: & Input,
	num_iters: u32,
) -> GenResult <u32> {
	let mut grid = get_grid (input) ?;
	let base_dirs: Vec <Pos> = get_base_dirs ();
	for _ in 0 .. num_iters { grid = next_grid (& grid, & base_dirs) ?; }
	Ok (
		grid.values ()
			.filter (|& tile| tile == Tile::Active)
			.count ()
			.as_u32 ()
	)
}

#[ inline ]
fn next_grid <Pos: GenPos <DIMS>, const DIMS: usize> (
	grid: & Grid <Pos, DIMS>,
	base_dirs: & [Pos],
) -> GenResult <Grid <Pos, DIMS>> {
	let resized_grid = grid.resize (
		grid.native_origin ().map (|val| val + 1),
		grid.native_size ().map (|val| val + 2)) ?;
	let dirs: Vec <GridOffset <DIMS>> =
		base_dirs.iter ()
			.map (|& pos| resized_grid.offset (pos))
			.collect ();
	Ok (resized_grid.map (move |cur| {
		let adj_active = dirs.iter ()
			.filter_map (|& dir| cur.try_add (dir).map (|cur| cur.item ()).ok ())
			.fold (0_u32, |sum, tile| sum + u32::from (tile == Tile::Active));
		match (cur.item (), adj_active) {
			(Tile::Active, 2 | 3) | (Tile::Inactive, 3) => Tile::Active,
			_ => Tile::Inactive, 
		}
	}))
}

#[ inline ]
fn get_grid <Pos: GenPos <DIMS>, const DIMS: usize> (
	input: & Input,
) -> GenResult <Grid <Pos, DIMS>> {
	let input_size = input.grid.native_size ();
	if 8 < input_size [0] || 8 < input_size [1] {
		return Err ("Max grid size is 8×8".into ());
	}
	let mut grid = Grid::new (
		[ 0_isize; DIMS ],
		array::from_fn (|idx| if idx < 2 { input_size [idx] } else { 1 }));
	for (pos, tile) in input.grid.iter () {
		let pos = Pos::pos_from_array (array::from_fn (|idx|
			match idx { 0 => pos.x, 1 => pos.y, _ => Coord::ZERO }));
		grid.set (pos, tile);
	}
	Ok (grid)
}

#[ inline ]
fn get_base_dirs <Pos: GenPos <DIMS>, const DIMS: usize> (
) -> Vec <Pos> {
	iter::repeat ([ Coord::NEG_ONE, Coord::ZERO, Coord::ONE ])
		.take (DIMS)
		.multi_cartesian_product ()
		.map (|combo| Pos::pos_from_array (combo.try_into ().unwrap ()))
		.filter (|& pos| pos != Pos::ZERO)
		.collect ()
}
