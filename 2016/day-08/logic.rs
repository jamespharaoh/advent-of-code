use super::*;

use input::Input;
use model::Grid;
use model::Pos;
use model::Step;

pub fn part_one (input: & Input) -> GenResult <usize> {
	let grid = calc_result (input);
	Ok (grid.values ().filter (|& val| val).count ())
}

pub fn part_two (input: & Input) -> GenResult <String> {
	let grid = calc_result (input);
	let code = ocr::read_fixed (
		grid.iter ()
			.filter_map (|(pos, val)| val
				.then_some ((pos.row, pos.col))),
		(6, 5)) ?;
	Ok (code)
}

#[ must_use ]
pub fn calc_result (input: & Input) -> Grid {
	let mut grid: Grid =
		GridBuf::new_size (Pos::new (input.params.height, input.params.width));
	for & step in input.steps.iter () {
		apply_step (& mut grid, step);
	}
	grid
}

pub fn apply_step (grid: & mut Grid, step: Step) {
	match step {
		Step::Rect { width, height } => {
			for row in 0 .. cmp::min (height, grid.size ().row) {
				for col in 0 .. cmp::min (width, grid.size ().col) {
					grid.set (Pos { row, col }, true);
				}
			}
		},
		Step::RotateRow { row, dist } => {
			if grid.size ().row <= row { return }
			let temp: Vec <bool> =
				(0 .. grid.size ().col)
					.map (|col| grid.get (Pos { row, col }).unwrap ())
					.collect ();
			for old_col in 0 .. grid.size ().col {
				let col = (old_col + dist) % grid.size ().col;
				grid.set (Pos { row, col }, temp [old_col.pan_usize ()]);
			}
		},
		Step::RotateCol { col, dist } => {
			if grid.size ().col <= col { return }
			let temp: Vec <bool> =
				(0 .. grid.size ().row)
					.map (|row| grid.get (Pos { row, col }).unwrap ())
					.collect ();
			for old_row in 0 .. grid.size ().row {
				let row = (old_row + dist) % grid.size ().row;
				grid.set (Pos { row, col }, temp [old_row.pan_usize ()]);
			}
		},
	}
}
