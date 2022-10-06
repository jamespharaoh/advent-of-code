use super::*;

use input::Input;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.moves.iter ()
			.scan (Pos::ZERO, |pos, & dir| { * pos += Pos::from (dir); Some (* pos) })
			.chain (iter::once (Pos::ZERO))
			.collect::<HashSet <_>> ()
			.len ()
			.pan_u32 ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		input.moves.iter ()
			.scan ((Pos::ZERO, Pos::ZERO), |& mut (ref mut pos_0, ref mut pos_1), & dir| {
				mem::swap (pos_0, pos_1);
				* pos_0 += Pos::from (dir);
				Some (* pos_0)
			})
			.chain (iter::once (Pos::ZERO))
			.collect::<HashSet <_>> ()
			.len ()
			.pan_u32 ()
	)
}
