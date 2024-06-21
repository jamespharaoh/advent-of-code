use super::*;

use input::Input;
use model::Pos;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result (input, 0)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	calc_result (input, 1)
}

#[ allow (clippy::range_minus_one) ]
fn calc_result (input: & Input, num_diffs: u32) -> GenResult <u32> {
	let mut rows_sum = 0;
	let mut cols_sum = 0;
	for grid in & input.grids {
		let rows: Vec <Vec <Tile>> =
			(grid.start ().row .. grid.end ().row)
				.map (|row| (grid.start ().col .. grid.end ().col)
					.map (|col| grid.get (Pos::new (row, col)).unwrap ())
					.collect ())
				.collect ();
		for row in 1 ..= rows.len () - 1 {
			if compare (& rows [ .. row], & rows [row .. ]) == num_diffs {
				chk! (rows_sum += row.pan_u32 ()) ?;
			}
		}
		let cols: Vec <Vec <Tile>> =
			(grid.start ().col .. grid.end ().col)
				.map (|col| (grid.start ().row .. grid.end ().row)
					.map (|row| grid.get (Pos::new (row, col)).unwrap ())
					.collect ())
				.collect ();
		for col in 1 ..= cols.len () - 1 {
			if compare (& cols [ .. col], & cols [col .. ]) == num_diffs {
				chk! (cols_sum += col.pan_u32 ()) ?;
			}
		}
	}
	Ok (chk! (rows_sum * 100 + cols_sum) ?)
}

fn compare (before: & [Vec <Tile>], after: & [Vec <Tile>]) -> u32 {
	iter::zip (before.iter ().rev (), after.iter ())
		.map (|(before, after)| iter::zip (before, after)
			.filter (|& (& a, & b)| a != b)
			.count ()
			.pan_u32 ())
		.sum ()
}
