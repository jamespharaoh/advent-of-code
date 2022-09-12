//! Logic for solving the puzzles

use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <String> {
	let mut signal = input.data.clone ();
	if signal.len () < input.params.result_digits.as_usize () {
		return Err ("Not enough signal data".into ());
	}
	if input.params.max_signal.as_usize () < signal.len () {
		return Err ("Too much signal data".into ());
	}
	for _ in 0 .. input.params.num_iters {
		signal = apply_fft (& signal, 0) ?;
	}
	Ok (
		signal.iter ()
			.take (input.params.result_digits.as_usize ())
			.display_delim ("")
			.to_string ()
	)
}

pub fn part_two (input: & Input) -> GenResult <String> {
	let offset =
		input.data.iter ().copied ()
			.take (input.params.offset_digits.as_usize ())
			.try_fold (0, |sum, item| u32::add_2 (u32::mul_2 (sum, 10) ?, item.as_u32 ())) ?;
	let signal_len = usize::sub_2 (
		input.data.len () * input.params.num_reps.as_usize (),
		offset.as_usize (),
	) ?;
	if signal_len < input.params.result_digits.as_usize () {
		return Err ("Not enough signal data".into ());
	}
	if input.params.max_signal.as_usize () < signal_len {
		return Err (format! ("Too much signal data {signal_len}").into ());
	}
	let mut signal: Vec <u8> =
		iter::repeat (& input.data)
			.flatten ()
			.skip (offset.as_usize () % input.data.len ())
			.take (signal_len)
			.copied ()
			.collect ();
	for _ in 0 .. input.params.num_iters {
		signal = apply_fft (& signal, offset) ?;
	}
	Ok (
		signal.iter ()
			.take (input.params.result_digits.as_usize ())
			.display_delim ("")
			.to_string ()
	)
}

#[ allow (clippy::unnecessary_wraps) ]
fn apply_fft (data: & [u8], offset: u32) -> GenResult <Vec <u8>> {
	let mut buffer: Vec <i32> =
		data.iter ().rev ()
			.map (|& val| val.as_i32 ())
			.scan (0_i32, |sum, val| { * sum += val; Some (* sum) })
			.collect ();
	buffer.reverse ();
	let result = (offset .. )
		.take (data.len ())
		.map (|dst_idx| {
			let sum = buffer [(dst_idx - offset).as_usize () .. ].iter ()
				.step_by ((dst_idx + 1).as_usize ())
				.enumerate ()
				.map (|(iter_idx, & val)| if (iter_idx + 1) & 0x2 == 0 { val } else { - val })
				.sum::<i32> ();
			(sum.unsigned_abs () % 10).as_u8 ()
		})
		.collect ();
	Ok (result)
}
