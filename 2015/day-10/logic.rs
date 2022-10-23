use super::*;

use input::Input;
use model::State;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let state = State::parse (& input.initial) ?;
	Ok (calc_result (state.iter ().copied (), input.params.num_iters_one))
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let state = State::parse (& input.initial) ?;
	Ok (calc_result (state.iter ().copied (), input.params.num_iters_two))
}

fn calc_result (iter: impl Iterator <Item = u8>, num_iters: u32) -> u32 {
	let mut iter: Box <dyn Iterator <Item = u8>> = Box::new (iter);
	for _ in 0 .. num_iters {
		iter = Box::new (make_iter (iter));
	}
	iter.count ().pan_u32 ()
}

//#[ allow (clippy::unused_peekable) ] // clippy is being dumb
fn make_iter (inner: impl Iterator <Item = u8>) -> impl Iterator <Item = u8> {
	let mut inner = inner.peekable ();
	let mut last = 0xff;
	let mut count = 0_u8;
	iter::from_fn (move || {
		while let Some (& next) = inner.peek () {
			if next != last && last != 0xff {
				let result = [count.pan_u8 (), last];
				last = 0xff;
				count = 0_u8;
				return Some (result);
			}
			inner.next ().unwrap ();
			last = next;
			if count < 9 { count += 1; }
		}
		if last != 0xff {
			let result = [count.pan_u8 (), last];
			last = 0xff;
			count = 0;
			return Some (result);
		}
		None
	}).flatten ()
}

#[ must_use ]
pub fn one_round (state: & State) -> State {
	state.iter ()
		.map (|& val| (val, 1_u8))
		.merge_consecutive (|(left, num_left), (right, num_right)|
			if left == right { Ok ((left, num_left + 1)) }
			else { Err (((left, num_left), (right, num_right))) })
		.flat_map (|(val, num)| [ num, val ])
		.collect ()
}
