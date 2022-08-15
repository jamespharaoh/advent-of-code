//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Dir;
use model::Slot;
use model::State;

pub fn part_one (input: & Input) -> GenResult <u32> {
//println! ("{input}");
	let states: HashMap <char, State> =
		input.states.iter ().copied ()
			.map (|state| (state.id, state))
			.collect ();
	if ! states.contains_key (& input.begin_state) {
		return Err (format! ("Start state {} does not exist", input.begin_state).into ());
	}
	if let Some ((src_id, dst_id)) =
		input.states.iter ()
			.flat_map (|state| [ (state.id, state.true_state), (state.id, state.false_state) ])
			.find (|& (_, dst_id)| ! states.contains_key (& dst_id)) {
		return Err (format! ("State {src_id} points to nonexistent state {dst_id}").into ());
	}
	let mut left = Vec::new ();
	let mut right = Vec::new ();
	let mut cur = Slot::Zero;
	let mut state_id = input.begin_state;
	for _ in 0 .. input.num_steps {
		let state = & states [& state_id];
		let (write, dir, new_state_id) = match cur {
			Slot::Zero => (state.false_write, state.false_dir, state.false_state),
			Slot::One => (state.true_write, state.true_dir, state.true_state),
		};
		match dir {
			Dir::Left => {
				right.push (write);
				cur = left.pop ().unwrap_or (Slot::Zero);
			},
			Dir::Right => {
				left.push (write);
				cur = right.pop ().unwrap_or (Slot::Zero);
			},
		}
		state_id = new_state_id;
	}
	Ok (
		iter::empty ()
			.chain (left.iter_vals ())
			.chain (iter::once (cur))
			.chain (right.iter_vals ())
			.filter (|& val| val == Slot::One)
			.count ()
			.as_u32 ()
	)
}
