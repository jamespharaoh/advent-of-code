use super::*;

use input::Input;
use model::Grid;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	Ok (
		step_iter (input.grid.clone ())
			.take (100)
			.sum ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let num_octopodes = input.grid.len ().pan_u32 ();
	let mut iter = step_iter (input.grid.clone ());
	let mut num_iters = 0;
	loop {
		if num_iters == 500 {
			return Err ("Giving up after 500 iterations".into ());
		}
		num_iters += 1;
		let num_flashes = iter.next ().unwrap ();
		if num_flashes == num_octopodes { return Ok (num_iters) }
	}
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.grid.size ().x < 2 || input.grid.size ().y < 2 {
		return Err ("Grid must be at least 2×2".into ());
	}
	if 100 < input.grid.size ().x || 100 < input.grid.size ().y {
		return Err ("Grid must be at most 50×50".into ());
	}
	Ok (())
}

fn step_iter (grid: Grid) -> impl Iterator <Item = u32> {
	let offsets: [GridOffset <Pos, 2>; 8] =
		Pos::ZERO.adjacent_8 ().into_iter ()
			.map (|pos| grid.offset (pos).unwrap ())
			.array ();
	iter::repeat (()).scan (grid, move |grid, _| {
		let mut num_flashes: u32 = 0;
		let mut flashed: GridBuf <Vec <bool>, Pos, 2> = GridBuf::new_range (grid.start (), grid.end ()).unwrap ();
		let mut todo: Vec <GridCursor <Pos, 2>> = Vec::new ();
		* grid = grid.map (|cur| {
			let energy = cur.get (& * grid);
			if energy < 9 { return energy + 1 }
			flashed.set_index (cur.index (), true);
			todo.push (cur);
			num_flashes += 1;
			0
		});
		while let Some (cur) = todo.pop () {
			for adj_off in offsets {
				let adj_cur = ok_or! (chk! (cur + adj_off), continue);
				let adj_energy = adj_cur.get (& * grid);
				if adj_cur.get (& flashed) { continue }
				if adj_energy < 9 {
					grid.set_index (adj_cur.index (), adj_energy + 1);
					continue;
				}
				flashed.set_index (adj_cur.index (), true);
				todo.push (adj_cur);
				num_flashes += 1;
				grid.set_index (adj_cur.index (), 0);
			}
		}
		Some (num_flashes)
	})
}
