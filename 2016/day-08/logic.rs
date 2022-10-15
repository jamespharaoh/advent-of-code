use super::*;

use input::Input;
use model::Step;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <usize> {
	let grid = calc_result (input);
	Ok (grid.values ().filter (|& val| val).count ())
}

pub fn part_two (input: & Input) -> GenResult <String> {
	let grid = calc_result (input);
	let code = ocr::read_dots (
		grid.iter ()
			.filter_map (|(pos, val)| val
				.then_some ((pos.row, pos.col)))) ?;
	Ok (code)
}

fn calc_result (input: & Input) -> GridBuf <Vec <bool>, Pos, 2> {
	let mut grid: GridBuf <Vec <bool>, Pos, 2> =
		GridBuf::new_size (Pos::new (input.params.height, input.params.width));
	for & step in input.steps.iter () {
		match step {
			Step::Rect { width, height } => {
				for row in 0 .. cmp::min (height, input.params.height) {
					for col in 0 .. cmp::min (width, input.params.width) {
						grid.set (Pos { row, col }, true);
					}
				}
			},
			Step::RotateRow { row, dist } => {
				if input.params.height <= row { continue }
				let temp: Vec <bool> =
					(0 .. input.params.width)
						.map (|col| grid.get (Pos { row, col }).unwrap ())
						.collect ();
				for old_col in 0 .. input.params.width {
					let col = (old_col + dist) % input.params.width;
					grid.set (Pos { row, col }, temp [old_col.pan_usize ()]);
				}
			},
			Step::RotateCol { col, dist } => {
				if input.params.width <= col { continue }
				let temp: Vec <bool> =
					(0 .. input.params.height)
						.map (|row| grid.get (Pos { row, col }).unwrap ())
						.collect ();
				for old_row in 0 .. input.params.height {
					let row = (old_row + dist) % input.params.height;
					grid.set (Pos { row, col }, temp [old_row.pan_usize ()]);
				}
			},
		}
	}
	grid
}
