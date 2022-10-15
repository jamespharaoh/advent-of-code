use super::*;

use input::Input;
use model::Dir;
use model::Pos;
use model::Turn;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let (pos, _) = iter_posns ().zip (1 .. ).find (|& (_, val)| val == input.target).unwrap ();
	Ok ((pos.row.unsigned_abs () + pos.col.unsigned_abs ()).pan_u32 ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let mut filled: HashMap <Pos, u32> = default ();
	filled.insert (Pos::ZERO, 1);
	for pos in iter_posns ().skip (1) {
		let next = pos.adjacent_8 ().iter ()
			.filter_map (|adj_pos| filled.get (adj_pos))
			.sum ();
		if input.target < next { return Ok (next) }
		filled.insert (pos, next);
	}
	Err ("No solution found".into ())
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.target < 1 { return Err ("Target must be at least one".into ()) }
	if input.target > 1_000_000 { return Err ("Target must be at most one million".into ()) }
	Ok (())
}

fn iter_posns () -> impl Iterator <Item = Pos> {
	let mut dir = Dir::Down;
	let mut pos = Pos::ZERO;
	let mut rem = 0_u32;
	let mut stride = 0_u32;
	iter::from_fn (move || {
		let next_pos = pos;
		if rem == 0 {
			dir = dir + Turn::Left;
			if matches! (dir, Dir::Right | Dir::Left) { stride += 1; }
			rem = stride;
		}
		rem -= 1;
		let dir_one = (dir, 1);
		pos = chk! (pos + dir_one).ok () ?;
		Some (next_pos)
	})
}
