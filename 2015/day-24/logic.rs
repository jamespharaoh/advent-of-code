//! Logic for solving the puzzles.

use super::*;

use input::Input;
use nums::Int;

pub fn part_one (input: & Input) -> GenResult <u64> {
	calc_result::<3> (input)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	calc_result::<4> (input)
}

fn calc_result <const PILES: usize> (input: & Input) -> GenResult <u64> {

	// make sure the weights are in reverse order

	let weights: Vec <u32> =
		input.weights.iter ().copied ()
			.sorted_by_key (|& weight| cmp::Reverse (weight))
			.collect ();

	// sanity check (mainly for fuzzing)

	if weights.len () > 50 {
		Err ("Refusing to deal with more than 50 items") ?;
	}

	if weights.iter ().dedup ().count () != weights.len () {
		Err ("Refusing to deal with duplicated weights") ?;
	}

	if weights.iter ().copied ().any (|weight| weight > 200) {
		Err ("Refusing to deal with weights over 200") ?;
	}

	// work out the balanced weight of each pile

	let total_weight: u32 =
		weights.iter ().copied ()
			.fold (Ok (0), |sum, item| sum
				.and_then (|sum| u32::add_2 (sum, item))) ?;
	let want_pile_weight = total_weight / PILES.pan_u32 ();
	if want_pile_weight * PILES.pan_u32 () != total_weight {
		Err (format! ("Total weight is not a multiple of {}", PILES)) ?;
	}

	// once we find one solution we can rule out any first piles which are bigger or have a
	// higher quantum entanglement

	let mut max_len_0 = usize::MAX;
	let mut max_quantum_0 = u64::MAX;

	// stack holds a list of piles and the indexes of their contents

	let mut stack: Vec <Vec <usize>> = vec! [ vec! [] ];

	// todo holds continuations, first value is number of piles to retain, second is number of
	// items in top pile, third is new index to push to top pile, for initial state we have a
	// single pile and branch for the full list of weights as the next item in it

	let mut todo: Vec <(usize, usize, usize)> =
		(0 .. weights.len ())
			.rev ()
			.map (|idx| (1, 0, idx))
			.collect ();

	// iterate through continuations, shortcircuit allows us to quickly get back to the first
	// pile when we find a new solution, because we don't really care about the other piles, so
	// long as we know there's at least one solution for them

	let mut shortcircuit = false;
	while let Some ((trunc_0, trunc_1, idx)) = todo.pop () {
		if shortcircuit && trunc_0 > 1 { continue }
		shortcircuit = false;

		// truncate the number of piles according to the continuation

		stack.truncate (trunc_0);

		// truncate the items in the top pile according to the continuation

		let pile_stack = stack.last_mut ().unwrap ();
		pile_stack.truncate (trunc_1);

		// add on the next item from the continuation

		pile_stack.push (idx);
		let pile_stack_len = pile_stack.len ();

		// work out the top pile's weight, abort if it is too heavy

		let pile_weight: u32 =
			pile_stack.iter ().copied ().map (|idx| weights [idx]).sum ();
		if pile_weight > want_pile_weight { continue }

		// for the first pile only, check the quantum and abort if it's already too high

		if stack.len () == 1 {
			if max_len_0 < stack [0].len () { continue }
			let quantum_0 =
				stack [0].iter ().copied ()
					.map (|idx| weights [idx].pan_u64 ())
					.fold (Ok (1), |prod, item| prod
						.and_then (|prod| u64::mul_2 (prod, item))) ?;
			if max_quantum_0 <= quantum_0 { continue }
		}

		// if this pile is now the right weight we start the next pile, or record a solution if
		// this is the last pile

		if pile_weight == want_pile_weight {
			if stack.len () < PILES {
				stack.push (vec! []);
				for idx in (0 .. weights.len ()).rev () {
					if stack.iter ()
						.any (|pile_stack| pile_stack.iter ().copied ()
							.any (|existing_idx| idx == existing_idx))
						{ continue }
					todo.push ((stack.len (), 0, idx));
				}
			} else {
				max_len_0 = stack [0].len ();
				max_quantum_0 =
					stack [0].iter ().copied ()
						.map (|idx| weights [idx].pan_u64 ())
						.fold (Ok (1), |prod, item| prod
							.and_then (|prod| u64::mul_2 (prod, item))) ?;
				shortcircuit = true;
			}
			continue;
		}

		// branch out for every possible next weight to add to the top pile

		let min_idx =
			stack.last ().unwrap ().iter ().copied ()
				.map (|idx| idx + 1).max ().unwrap_or (0);
		for idx in (min_idx .. weights.len ()).rev () {
			if stack.iter ()
				.any (|pile_stack| pile_stack.iter ().copied ()
					.any (|existing_idx| idx == existing_idx))
				{ continue }
			todo.push ((stack.len (), pile_stack_len, idx));
		}

	}

	if max_quantum_0 == u64::MAX { Err ("No solution found") ?; }
	Ok (max_quantum_0)

}

#[ cfg (test) ]
mod tests {

	use super::*;

	use input::InputParams;

	#[ test ]
	fn calc_result () {
		fn invoke <const PIL: usize> (weights: impl IntoIterator <Item = u32>) -> GenResult <u64> {
			let input = Input {
				weights: weights.into_iter ().collect (),
				params: InputParams::default (),
			};
			logic::calc_result::<PIL> (& input)
		}
		assert_eq_ok! (5, invoke::<3> (1 ..= 5));
		assert_err! ("No solution found", invoke::<3> (1 ..= 3));
		assert_err! ("Refusing to deal with more than 50 items", invoke::<3> (0 ..= 50));
		assert_err! ("Refusing to deal with duplicated weights", invoke::<3> ([ 1, 1, 1 ]));
		assert_err! ("Total weight is not a multiple of 3", invoke::<3> ([ 1, 2, 3, 4 ]));
	}

}
