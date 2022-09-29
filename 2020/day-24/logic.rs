//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Coord;
use model::Grid;
use model::Pos;
use model::Tile::{ self, White, Black };

pub fn part_one (input: & Input) -> GenResult <u32> {
	let grid = get_start_grid (input) ?;
	Ok (count_black (& grid))
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	const EXTEND_AMT: Coord = 1;
	const EXTEND_AMTS: [(Coord, Coord); 2] = [(EXTEND_AMT, EXTEND_AMT); 2];
	let mut grid = get_start_grid (input) ?;
	let mut needs_extend = true;
	for _ in 0_u32 .. input.params.num_iters {
		//println! ("\x1bc{grid}");
		//thread::sleep (time::Duration::from_micros (200_000));
		(grid, needs_extend) = if needs_extend {
			next_grid (& grid.extend (EXTEND_AMTS) ?) ?
		} else {
			next_grid (& grid) ?
		};
	}
	Ok (count_black (& grid))
}

fn next_grid (grid: impl GridView <Pos, 2, Item = Tile>) -> GenResult <(Grid, bool)> {
	let offsets: ArrayVec <_, 6> =
		Pos::ZERO.adjacent ().into_iter ()
			.map (|pos| grid.offset (pos))
			.try_collect () ?;
	let mut needs_extend = false;
	let grid = grid.map (|cur| {
		let (num_adjacent, num_total) =
			offsets.iter ()
				.filter_map (|& off| chk! (cur + off).map (|cur| cur.item ()).ok ())
				.fold ((0_u32, 0_u32), |(num_adj, num_tot), item|
					(num_adj + u32::from (item == Black), num_tot + 1));
		let black = matches! ((cur.item (), num_adjacent), (Black, 1 ..= 2) | (White, 2));
		if black && num_total < 6 { needs_extend = true; }
		if black { Black } else { White }
	});
	Ok ((grid, needs_extend))
}

fn count_black (grid: & Grid) -> u32 {
	grid.values ()
		.filter (|& tile| tile == Black)
		.count ()
		.pan_u32 ()
}

fn get_start_grid (input: & Input) -> GenResult <Grid> {
	let tile_posns: Vec <Pos> = input.tiles.iter ()
		.map (|tile_steps| {
			let mut pos = Pos::ZERO;
			for & step in & ** tile_steps {
				pos = pos.try_add (step.into ()) ?;
			}
			Ok::<_, Overflow> (pos)
		})
		.try_collect () ?;
	let (min, max) = tile_posns.iter ()
		.fold ((Pos::ZERO, Pos::ZERO), |(min, max), & pos| (
			Pos::new (cmp::min (min.nw, pos.nw), cmp::min (min.e, pos.e)),
			Pos::new (cmp::max (max.nw, pos.nw), cmp::max (max.e, pos.e)),
		));
	if 30 < max.nw - min.nw || 30 < max.e - min.e {
		return Err ("Max initial grid size is 30×30".into ());
	}
	let mut grid = Grid::new_range (min, max + Pos::new (1, 1)) ?;
	for tile_pos in tile_posns {
		let tile_prev = grid.get (tile_pos).unwrap ();
		let tile_next = match tile_prev { White => Black, Black => White };
		grid.set (tile_pos, tile_next);
	}
	Ok (grid)
}

pub fn part_two_bits (input: & Input) -> GenResult <u32> {
	let grid = get_start_grid (input) ?;
	let mut grid = grid_to_bits (& grid);
	for _ in 0_u32 .. input.params.num_iters {
		if grid [0] != 0 || grid [grid.len () - 1] != 0
				|| grid.iter ().any (|& val| val & (1 << (grid.len () - 1)) != 0 || val & 1 != 0) {
			grid = iter::empty ()
				.chain (iter::once (0))
				.chain (grid)
				.chain (iter::once (0))
				.map (|val| val << 1_u32)
				.collect ();
		}
		grid = next_grid_bits (& grid);
	}
	Ok (grid.iter ().map (|& val| val.count_ones ()).sum ())
}

fn grid_to_bits (grid: & Grid) -> Vec <u128> {
	let cur = grid.cursor (grid.first_key ()).unwrap ();
	let north_west = grid.offset (Pos::ZERO.north_west (1).unwrap ()).unwrap ();
	let east = grid.offset (Pos::ZERO.east (1).unwrap ()).unwrap ();
	cur.walk (north_west)
		.map (|cur| cur.walk (east)
			.fold (0_u128, |sum, cur| sum << 1_u32 | u128::from (cur.item () == Black)))
		.collect ()
}

#[ allow (clippy::unusual_byte_groupings) ]
fn next_grid_bits (grid: & [u128]) -> Vec <u128> {
	let grid: Vec <u128> = iter::empty ()
		.chain (iter::once (0))
		.chain (grid.iter ().copied ())
		.chain (iter::once (0))
		.map (|val| val << 1_u32)
		.tuple_windows ()
		.map (|(mut a, mut b, mut c)| {
			let mut row = 0;
			for _ in 0 .. grid.len () {
				let black = b & 0b_010 != 0;
				let num_adj = (a.qck_u8 () & 0b_110).count_ones ()
					+ (b.qck_u8 () & 0b_101).count_ones ()
					+ (c.qck_u8 () & 0b_011).count_ones ();
				let black = matches! ((black, num_adj), (true, 1 ..= 2) | (false, 2));
				row = row >> 1_u32 | (u128::from (black) << (grid.len () - 1));
				a >>= 1_u32;
				b >>= 1_u32;
				c >>= 1_u32;
			}
			row
		})
		.collect ();
	grid
}
