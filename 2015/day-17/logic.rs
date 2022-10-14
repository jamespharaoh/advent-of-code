//! Logic for solving the puzzles.

use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let num_combos = combos (input).count ();
	Ok (num_combos.pan_u32 ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let (_, num) =
		combos (input)
			.fold ((u32::MAX, 0), |(min, num), combo_len|
				match combo_len.cmp (& min) {
					Ordering::Less => (combo_len, 1),
					Ordering::Equal => (min, num + 1),
					Ordering::Greater => (min, num),
				});
	Ok (num)
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.sizes.len () > 20 {
		return Err ("Refusing to deal with more than 20 containers".into ());
	}
	let sum = input.sizes.iter ().try_fold (0, |sum, & item| chk! (sum + item)) ?;
	if sum < input.params.target {
		return Err ("No solution found".into ());
	}
	Ok (())
}

/// Iterate over combinations of provided container given a specific total.
///
/// Returns an [`Iterator`] over the number of containers in each combination.
///
pub fn combos (input: & Input) -> impl Iterator <Item = u32> + '_ {
	let sizes: Vec <u32> =
		input.sizes.iter ().copied ()
			.sorted_by_key (|& size| cmp::Reverse (size))
			.collect ();
	let target = input.params.target;
	let mut state = iter::repeat (false).take (sizes.len ()).collect::<Vec <_>> ();
	let mut sum = 0;
	iter::from_fn (move || loop {
		if state.is_empty () { return None }
		let mut idx = state.len () - 1;
		loop {
			if state [idx] {
				state [idx] = false;
				sum -= sizes [idx];
				if idx > 0 { idx -= 1 } else { return None }
			} else if 150 < sum
					|| sum + sizes.iter ().copied ().skip (idx).sum::<u32> () < target {
				if idx > 0 { idx -= 1 } else { return None }
			} else { break }
		}
		state [idx] = true;
		sum += sizes [idx];
		if sum == target {
			return Some (state.iter ().filter (|&& state| state).count ().pan_u32 ())
		}
	})
}
