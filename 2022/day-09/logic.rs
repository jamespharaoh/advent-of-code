use super::*;

use input::Input;
use model::Coord;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result::<2> (input)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	calc_result::<10> (input)
}

fn calc_result <const NUM_KNOTS: usize> (input: & Input) -> GenResult <u32> {
	let mut knots = [Pos::ZERO; NUM_KNOTS];
	let mut tail_posns = HashSet::new ();
	tail_posns.insert (Pos::ZERO);
	for step in & input.steps {
		for _ in 0 .. step.num {
			chk! (knots [0] += step.dir.into ()) ?;
			for head_idx in 0 .. NUM_KNOTS - 1 {
				let head_pos = knots [head_idx];
				move_tail (& mut knots [head_idx + 1], head_pos);
			}
			tail_posns.insert (knots [NUM_KNOTS - 1]);
		}
	}
	Ok (tail_posns.len ().pan_u32 ())
}

pub fn move_tail (tail: & mut Pos, head: Pos) {
	if Coord::abs_diff (tail.y, head.y) < 2 && Coord::abs_diff (tail.x, head.x) < 2 {
		return;
	}
	match tail.y.cmp (& head.y) {
		Ordering::Less => { tail.y += 1; },
		Ordering::Equal => (),
		Ordering::Greater => { tail.y -= 1; },
	}
	match tail.x.cmp (& head.x) {
		Ordering::Less => { tail.x += 1; },
		Ordering::Equal => (),
		Ordering::Greater => { tail.x -= 1; },
	}
}
