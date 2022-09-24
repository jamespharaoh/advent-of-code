//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Grid;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {

	sanity_check (input) ?;

	// work out size

	let height = input.posns.iter_vals ().map (|pos| pos.y + 1_i32).max ().unwrap ();
	let width = input.posns.iter_vals ().map (|pos| pos.x + 1_i32).max ().unwrap ();

	// initialize grid

	let mut grid = Grid::new ([0, 0], [height.as_usize (), width.as_usize ()]);
	let mut next_id: u8 = 1;
	for pos in input.posns.iter_vals () {
		if next_id == u8::MAX { panic! () }
		grid.set (pos, next_id);
		next_id += 1;
	}

	// spread out one square at a time

	let mut todo: Vec <(Pos, u8)> =
		input.posns.iter_vals ()
			.flat_map (|pos| {
				let val = grid.get (pos).unwrap ();
				pos.adjacent_4 ().iter_vals ()
					.filter (|& adj_pos| grid.get (adj_pos).map_or (false, |val| val == 0))
					.map (|adj_pos| (adj_pos, val))
					.collect::<ArrayVec <(Pos, u8), 4>> ()
			})
			.collect ();

	let mut todo_next: Vec <(Pos, u8)> = Vec::new ();
	let mut adj_vals: Vec <u8> = Vec::new ();

	loop {
		todo.sort ();
		let mut progress = false;
		let temp = todo.drain ( .. ).group_by (|& (pos, _)| pos);
		for (pos, group_iter) in temp.into_iter () {
			let val = some_or! (grid.get (pos), continue);
			if val != 0 { continue }
			adj_vals.clear ();
			adj_vals.extend (group_iter.map (|(_, val)| val));
			let first = adj_vals [0];
			if adj_vals.iter_vals ().all (|adj_val| adj_val == first) {
				let new_val = adj_vals [0];
				grid.set (pos, new_val);
				for adj_pos in pos.adjacent_4 () {
					if let Some (adj_val) = grid.get (adj_pos) {
						if adj_val == 0 { todo_next.push ((adj_pos, new_val)); }
					}
				}
			} else {
				grid.set (pos, u8::MAX);
			}
			progress = true;
		}
		if ! progress { break }
		drop (temp);
		mem::swap (& mut todo, & mut todo_next);
	}

	// measure size of areas

	let mut areas = [0_u32; 256];

	for value in grid.values () {
		if value == 0 || value == u8::MAX { continue }
		areas [value.as_usize ()] += 1;
	}

	// remove areas which reach the edge

	for (_, val) in grid.iter ()
		.filter (|& (pos, _)|
			pos.x == 0_i32 || pos.x == width - 1_i32
				|| pos.y == 0_i32 || pos.y == height - 1_i32) {
		areas [val.as_usize ()] = 0;
	}

	// find largest remaining area

	Ok (areas.iter_vals ().max ().unwrap ())

}

pub fn part_two (input: & Input) -> GenResult <u32> {

	sanity_check (input) ?;

	// work out size

	let height = input.posns.iter_vals ().map (|pos| pos.y + 1_i32).max ().unwrap ();
	let width = input.posns.iter_vals ().map (|pos| pos.x + 1_i32).max ().unwrap ();

	// work out horizontal/vertical distances separately

	fn calc_axis_dists (posns_iter: impl Iterator <Item = i32>, size: i32) -> Vec <u32> {
		let posns: Vec <i32> = posns_iter.sorted ().collect ();
		fn calc_one_way (
			posns_iter: impl Iterator <Item = i32>,
			range: impl Iterator <Item = i32>,
		) -> Vec <u32> {
			let mut num_points = 0;
			let mut cur_dist = 0;
			let mut posns_iter = posns_iter.peekable ();
			let mut dists: Vec <u32> = Vec::new ();
			for pos in range {
				cur_dist += num_points;
				dists.push (cur_dist);
				while posns_iter.peek () == Some (& pos) {
					posns_iter.next ().unwrap ();
					num_points += 1;
				}
			}
			dists
		}
		let dists_fwd = calc_one_way (posns.iter_vals (), 0_i32 .. size);
		let dists_rev = calc_one_way (posns.iter_vals ().rev (), (0_i32 .. size).rev ());
		dists_fwd.iter_vals ().zip (dists_rev.iter_vals ().rev ())
			.map (|(fwd, rev)| fwd + rev)
			.collect ()
	}

	let dists_x = calc_axis_dists (input.posns.iter ().map (|pos| pos.x), width);
	let dists_y = calc_axis_dists (input.posns.iter ().map (|pos| pos.y), height);

	// sum separate distances to get totals and count how many are in range

	let mut area = 0_u32;
	for dist_x in dists_x.iter_vals () {
		if dist_x > input.params.dist_two { continue }
		for dist_y in dists_y.iter_vals () {
			if dist_x + dist_y > input.params.dist_two { continue }
			area += 1;
		}
	}

	Ok (area)

}

fn sanity_check (input: & Input) -> GenResult <()> {
	if input.posns.is_empty () { return Err ("Must have at least one position".into ()) }
	if input.posns.len () > 50 { return Err ("Refusing to handle more than 50 points".into ()) }
	if input.posns.iter_vals ()
			.any (|pos| ! (0_i32 ..= 399_i32).contains (& pos.x)
				|| ! (0_i32 ..= 399_i32).contains (& pos.y)) {
		return Err ("Refusing to handle positions not between 0 and 399".into ());
	}
	Ok (())
}
