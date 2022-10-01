use super::*;

use input::Input;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut sum: u32 = 0;
	'OUTER: for (pos, height) in input.grid.iter () {
		for next_pos in pos.adjacent_4 () {
			if let Some (next_height) = input.grid.get (next_pos) {
				if next_height <= height { continue 'OUTER }
			}
		}
		sum += height.pan_u32 () + 1;
	}
	Ok (sum)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let offsets: Vec <_> =
		Pos::ZERO.adjacent_4 ().into_iter ()
			.map (|pos| input.grid.offset (pos))
			.try_collect () ?;
	let mut basin_sizes: Vec <u32> = Vec::new ();
	let mut seen: Vec <bool> = iter::repeat (false).take (input.grid.len ()).collect ();
	for cur in input.grid.cursors () {
		if seen [cur.index ()] { continue }
		seen [cur.index ()] = true;
		if cur.item () == 9 { continue }
		let mut todo: VecDeque <_> = VecDeque::new ();
		todo.push_back (cur);
		let mut basin_size = 1_u32;
		while let Some (cur) = todo.pop_front () {
			for & offset in offsets.iter () {
				let adj_cur = ok_or! (chk! (cur + offset), continue);
				if seen [adj_cur.index ()] { continue }
				seen [adj_cur.index ()] = true;
				if adj_cur.item () == 9 { continue }
				todo.push_back (adj_cur);
				basin_size += 1;
			}
		}
		basin_sizes.push (basin_size);
	}
	basin_sizes.sort_unstable ();
	Ok (
		basin_sizes.into_iter ().rev ()
			.take (3)
			.try_fold (1_u32, |prod, val| chk! (prod * val)) ?
	)
}
