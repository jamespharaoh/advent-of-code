use super::*;

use input::Input;
use input::Step;
use model::Coord;
use model::Dir;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u64> {
	calc_result (& input.steps)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let steps: Vec <Step> =
		input.steps.iter ()
			.map (|step| -> GenResult <Step> {
				let num = step.colour / 16;
				if num < 1 { return Err ("Step distance must be at least 1".into ()); }
				Ok (Step {
					dir: match step.colour & 0x3 {
						0 => Dir::Right,
						1 => Dir::Down,
						2 => Dir::Left,
						3 => Dir::Up,
						_ => unreachable! (),
					},
					num,
					colour: step.colour,
				})
			})
			.try_collect () ?;
	calc_result (& steps)
}

pub fn calc_result (steps: & [Step]) -> GenResult <u64> {
	if steps.len () < 4 {
		return Err ("Must have at least 4 steps".into ());
	}
	let posns: Vec <_> =
		steps.iter ()
			.scan (Pos::ZERO, |pos, step| {
				* pos += Pos::from (step.dir) * step.num.pan_i32 ();
				Some (* pos)
			})
			.collect ();
	if * posns.last ().unwrap () != Pos::ZERO {
		return Err ("Must return to start".into ());
	}
	let mut sum = 0;
	let mut old_y = Coord::MIN;
	let mut gap_sum = 0_i64;
	for new_y in posns.iter ().map (|& pos| pos.y).sorted ().dedup_consecutive () {
		sum += gap_sum.pan_i64 () * (new_y.pan_i64 () - old_y.pan_i64 () - 1);
		let swaps: Vec <_> =
			iter::once (Pos::ZERO)
				.chain (posns.iter ().copied ())
				.array_windows ()
				.filter_map (|[ start, end ]| {
					if start.y == end.y { return None }
					assert! (start.x == end.x);
					let x = start.x;
					let min = cmp::min (start.y, end.y);
					let max = cmp::max (start.y, end.y);
					if max < new_y || new_y < min { return None }
					Some ((x, new_y < max, min < new_y))
				})
				.sorted_by_key (|& (x, _, _)| x)
				.collect ();
		let mut old_x = Coord::MIN;
		let mut in_above = false;
		let mut in_below = false;
		gap_sum = 0_i64;
		for (new_x, swap_below, swap_above) in swaps {
			if in_above || in_below { sum += new_x.pan_i64 () - old_x.pan_i64 () - 1; }
			sum += 1;
			if in_below { gap_sum += new_x.pan_i64 () - old_x.pan_i64 () - 1; }
			if in_below || swap_below { gap_sum += 1; }
			if swap_above { in_above = ! in_above; }
			if swap_below { in_below = ! in_below; }
			old_x = new_x;
		}
		assert! (! in_above && ! in_below);
		old_y = new_y;
	}
	Ok (sum.pan_u64 ())
}
