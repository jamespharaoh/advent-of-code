//! Logic for solving the puzzles

use super::*;

use input::Input;
use input::Shuffle;
use model::Operation;

pub fn part_one (input: & Input) -> GenResult <u64> {
	Ok (
		get_operation (input, input.params.deck_size_one) ?
			.apply (input.params.init_one)
	)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	Ok (
		get_operation (input, input.params.deck_size_two) ?
			.reverse ().ok_or ("Desk size must be prime") ?
			.repeat (input.params.repeat_two)
			.apply (input.params.init_two)
	)
}

fn get_operation (input: & Input, deck_size: u64) -> GenResult <Operation> {
	assert! (0 < deck_size);
	input.shuffles.iter ().copied ()
		.try_fold (Operation::new (deck_size), |op, shuffle| {
			let shuffle_op = shuffle_to_operation (shuffle, deck_size) ?;
			Ok (op.then (shuffle_op))
		})
}

fn shuffle_to_operation (shuffle: Shuffle, deck_size: u64) -> GenResult <Operation> {
	match shuffle {
		Shuffle::DealIntoNewStack => {
			Ok (
				Operation::new (deck_size)
					.then_multiply (deck_size - 1)
					.then_add (deck_size - 1)
			)
		},
		Shuffle::Cut (arg) => {
			let arg = if 0_i32 <= arg {
				u64::sub_2 (deck_size, arg.pan_u64 ()) ?
			} else {
				arg.unsigned_abs ().pan_u64 ()
			};
			if deck_size <= arg {
				return Err (format! (
					"Invalid shuffle {shuffle:?} for deck size {deck_size}").into ());
			}
			Ok (
				Operation::new (deck_size)
					.then_add (arg)
			)
		},
		Shuffle::DealWithIncrement (arg) => {
			let arg = arg.pan_u64 ();
			if deck_size <= arg {
				return Err (format! (
					"Invalid shuffle {shuffle:?} for deck size {deck_size}").into ());
			}
			Ok (
				Operation::new (deck_size)
					.then_multiply (arg)
			)
		},
	}
}
