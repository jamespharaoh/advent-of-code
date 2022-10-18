//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Coord;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {

	check_input (input) ?;

	// work out size

	let (start, end) = input.posns.iter ()
		.fold (None, |state: Option <(Pos, Pos)>, & pos| state
			.map (|(start, end)| (
				Pos::new (cmp::min (start.y, pos.y), cmp::min (start.x, pos.x)),
				Pos::new (cmp::max (end.y, pos.y), cmp::max (end.x, pos.x)),
			))
			.or (Some ((pos, pos))))
		.unwrap ();

	// work out area sizes, excluding any which touch the sides

	let mut areas = vec! [0_u32; input.posns.len ()];
	for y in start.y ..= end.y {
		for x in start.x ..= end.x {
			let pos = Pos::new (y, x);
			let (_, close_idx) = input.posns.iter ().enumerate ()
				.fold ((u16::MAX, usize::MAX), |(close_dist, close_idx), (inp_idx, inp_pos)| {
					let dist = inp_pos.y.abs_diff (pos.y) + inp_pos.x.abs_diff (pos.x);
					#[ allow (clippy::comparison_chain) ] // this is significantly quicker
					if dist < close_dist { (dist, inp_idx) }
					else if close_dist == dist { (close_dist, usize::MAX) }
					else { (close_dist, close_idx) }
				});
			if close_idx == usize::MAX { continue }
			if areas [close_idx] == u32::MAX { continue }
			if y == start.y || y == end.y || x == start.x || x == end.x {
				areas [close_idx] = u32::MAX;
			} else {
				areas [close_idx] += 1;
			}
		}
	}

	// find largest area

	Ok (
		areas.into_iter ()
			.filter (|& area| area != u32::MAX)
			.max ()
			.ok_or ("No solution found") ?
	)

}

pub fn part_two (input: & Input) -> GenResult <u32> {

	check_input (input) ?;

	// work out size

	let height = input.posns.iter ().map (|& pos| pos.y + Coord::ONE).max ().unwrap ();
	let width = input.posns.iter ().map (|& pos| pos.x + Coord::ONE).max ().unwrap ();

	// work out horizontal/vertical distances separately

	let dists_x = calc_axis_dists (
		input.posns.iter ().map (|pos| pos.x.pan_i32 ()),
		width.pan_i32 ());
	let dists_y = calc_axis_dists (
		input.posns.iter ().map (|pos| pos.y.pan_i32 ()),
		height.pan_i32 ());

	// sum separate distances to get totals and count how many are in range

	let mut area = 0_u32;
	for & dist_x in dists_x.iter () {
		if dist_x > input.params.dist_two { continue }
		for & dist_y in dists_y.iter () {
			if dist_x + dist_y > input.params.dist_two { continue }
			area += 1;
		}
	}

	Ok (area)

}

fn calc_axis_dists (posns_iter: impl Iterator <Item = i32>, size: i32) -> Vec <u32> {
	let posns: Vec <i32> = posns_iter.sorted ().collect ();
	let dists_fwd = calc_one_way (posns.iter ().copied (), 0_i32 .. size);
	let dists_rev = calc_one_way (posns.iter ().rev ().copied (), (0_i32 .. size).rev ());
	dists_fwd.iter ().zip (dists_rev.iter ().rev ())
		.map (|(& fwd, & rev)| fwd + rev)
		.collect ()
}

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

fn check_input (input: & Input) -> GenResult <()> {
	if input.posns.is_empty () { return Err ("Must have at least one position".into ()) }
	if input.posns.len () > 50 { return Err ("Refusing to handle more than 50 points".into ()) }
	Ok (())
}
