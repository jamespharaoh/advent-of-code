//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Coord;
use model::Point;

pub fn part_one (input: & Input) -> GenResult <String> {
	let (message, _) = calc_result (input) ?;
	Ok (message)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let (_, num_iters) = calc_result (input) ?;
	Ok (num_iters)
}

fn calc_result (input: & Input) -> GenResult <(String, u32)> {
	fn calc_range (points: & [Point]) -> NumResult <(Coord, Coord, Coord, Coord)> {
		points.iter ().copied ().fold (
			Ok ((Coord::MAX, Coord::MIN, Coord::MAX, Coord::MIN)),
			|state, point| state.and_then (|(min_y, max_y, min_x, max_x)| Ok ((
				cmp::min (min_y, point.pos.y),
				cmp::max (max_y, chk! (point.pos.y + Coord::ONE) ?),
				cmp::min (min_x, point.pos.x),
				cmp::max (max_x, chk! (point.pos.x + Coord::ONE) ?),
			))))
	}
	fn calc_size (points: & [Point]) -> NumResult <(Coord, Coord)> {
		let (min_y, max_y, min_x, max_x) = calc_range (points) ?;
		Ok ((chk! (max_y - min_y) ?, chk! (max_x - min_x) ?))
	}
	let mut points = input.points.clone ();
	let mut points_temp = Vec::new ();
	let (mut size_y, mut size_x) = calc_size (& points) ?;
	let mut num_iters = 0_u32;
	let mut step = 0x10000_u32;
	loop {
		points_temp.clear ();
		for new_point in points.iter ().map (|& point| point.offset (step.pan_i32 ())) {
			points_temp.push (new_point ?);
		}
		let (next_y, next_x) = calc_size (& points_temp) ?;
		if size_y < next_y && size_x < next_x {
			if step == 1 { break }
			if num_iters >= step {
				points_temp.clear ();
				for new_point in points.iter ()
						.map (|& point| point.offset (- step.pan_i32 ())) {
					points_temp.push (new_point ?);
				}
				mem::swap (& mut points, & mut points_temp);
				(size_y, size_x) = calc_size (& points) ?;
				num_iters -= step;
			}
			step >>= 1_u32;
			continue;
		}
		mem::swap (& mut points, & mut points_temp);
		(size_y, size_x) = (next_y, next_x);
		num_iters = chk! (num_iters + step) ?;
	}
	let message = ocr::read_dots (points.iter ()
		.map (|& point| (point.pos.y, point.pos.x))) ?;
	Ok ((message, num_iters))
}
