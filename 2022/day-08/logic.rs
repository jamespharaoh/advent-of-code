use super::*;

use input::Input;
use model::Dir;
use model::Grid;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let grid = & input.grid;
	let mut visible: Grid <bool> = Grid::new_range (grid.start (), grid.end ()) ?;
	let mut todo = Vec::new ();
	let offset_right = grid.offset (Dir::Right.into ()).unwrap ();
	let offset_left = grid.offset (Dir::Left.into ()).unwrap ();
	let offset_down = grid.offset (Dir::Down.into ()).unwrap ();
	let offset_up = grid.offset (Dir::Up.into ()).unwrap ();
	for y in grid.first_key ().y ..= grid.last_key ().y {
		todo.push ((grid.cursor (Pos::new (y, grid.first_key ().x)).unwrap (), offset_right, -1));
		todo.push ((grid.cursor (Pos::new (y, grid.last_key ().x)).unwrap (), offset_left, -1));
	}
	for x in grid.first_key ().x ..= grid.last_key ().x {
		todo.push ((grid.cursor (Pos::new (grid.first_key ().y, x)).unwrap (), offset_down, -1));
		todo.push ((grid.cursor (Pos::new (grid.last_key ().y, x)).unwrap (), offset_up, -1));
	}
	while let Some ((cur, dir, prev_val)) = todo.pop () {
		let this_val = cur.get (grid);
		if prev_val < this_val { visible.set (cur.pos (), true); }
		if let Ok (next_cur) = chk! (cur + dir) {
			todo.push ((next_cur, dir, cmp::max (prev_val, this_val)));
		}
	}
	Ok (
		visible.values ()
			.filter (|& val| val)
			.count ()
			.pan_u32 ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let grid = & input.grid;
	let dirs = [
		grid.offset (Dir::Right.into ()).unwrap (),
		grid.offset (Dir::Left.into ()).unwrap (),
		grid.offset (Dir::Down.into ()).unwrap (),
		grid.offset (Dir::Up.into ()).unwrap (),
	];
	Ok (
		grid.cursors ()
			.map (|start_cur| {
				let start_val = start_cur.get (grid);
				dirs.into_iter ()
					.map (|dir| {
						let mut num = 0;
						for walk_cur in start_cur.walk (dir).skip (1) {
							num += 1;
							if start_val <= walk_cur.get (grid) { break }
						}
						num
					})
					.product ()
			})
			.max ()
			.ok_or ("No solution found") ?
	)
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.grid.size ().x < 2 || input.grid.size ().y < 2 {
		return Err ("Grid size must be at least 2×2".into ());
	}
	Ok (())
}
