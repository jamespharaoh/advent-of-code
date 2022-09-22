//! Logic for solving the puzzles

use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result (input, input.params.ord_one)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	calc_result (input, input.params.ord_two)
}

fn calc_result (input: & Input, ord: u32) -> GenResult <u32> {
	Ok (
		game_iter (& input.start_nums) ?
			.nth ((ord - 1).as_usize ())
			.unwrap ()
	)
}

fn game_iter (start: & [u32]) -> GenResult <impl Iterator <Item = u32> + '_> {
	if start.is_empty () { return Err ("Must have at least one starting number".into ()) }
	let mut posns: HashMap <u32, u32> =
		start.iter ().enumerate ()
			.map (|(idx, & num)| (num, idx.as_u32 ()))
			.collect ();
	let mut pos = start.len ().as_u32 () - 1;
	let & (mut next) = start.last ().unwrap ();
	Ok (
		start.iter ().copied ()
			.take (pos.as_usize ())
			.chain (iter::from_fn (move || {
				let num = next;
				let & prev_pos = posns.get (& num).unwrap_or (& pos);
				next = pos - prev_pos;
				posns.insert (num, pos);
				pos += 1;
				Some (num)
			}))
	)
}
