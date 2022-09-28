//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Generator;
use model::Pot;
use model::State;

pub fn part_one (input: & Input) -> GenResult <i64> {
	calc_result (input, 20_u64)
}

pub fn part_two (input: & Input) -> GenResult <i64> {
	calc_result (input, 50_000_000_000_u64)
}

fn calc_result (input: & Input, num_iters: u64) -> GenResult <i64> {

	// set up various things

	let generator = Generator::build (input) ?;
	let mut state = State::from (input.start.as_slice ());
	let mut cache: HashMap <Vec <u8>, (u64, i64)> = HashMap::new ();

	// loop for the specified number of iterations

	let mut cur_iters = 0_u64;
	while cur_iters < num_iters {
		state = generator.next (& state) ?;
		cur_iters += 1;

		// if we revisit the same state, even in a different position, take a short cut using some
		// simple maths

		if let Some (& (prev_iters, prev_start)) = cache.get (& state.data) {
			let loop_iters = u64::sub_2 (cur_iters, prev_iters) ?;
			let rem_iters = u64::sub_2 (num_iters, cur_iters) ?;
			if loop_iters <= rem_iters {
				let reps = u64::div_2 (rem_iters, loop_iters) ?;
				state.start = i64::add_2 (state.start, Int::mul_2 (
					Int::sub_2 (state.start, prev_start) ?,
					reps.pan_i64 (),
				) ?) ?;
				cur_iters = Int::add_2 (cur_iters, Int::mul_2 (reps, loop_iters) ?) ?;
			}
		}

		// update cache, abort if things are taking too long

		cache.entry (state.data.clone ()).or_insert ((cur_iters, state.start));
		if cache.len () == 200 { return Err ("Giving up after 200 distinct states".into ()) }

	}

	// calculate the answer from the final state and return

	Ok (
		state.iter ()
			.map (|(pos, pot)| {
				Ok (match pot {
					Pot::Empty => 0,
					Pot::Plant => pos.to_i64 () ?,
				})
			})
			.fold (Ok (0), |sum, item| Int::add_2 (sum ?, item ?)) ?
	)

}
