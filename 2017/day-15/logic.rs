use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	#[ inline ]
	fn calc_next (factor: u64, prev: u64) -> u64 {
		prev.pan_u64 ().wrapping_mul (factor.pan_u64 ()) % 0x_7fff_ffff
	}
	Ok (
		calc_result (
			input,
			input.params.reps_one,
			|val| calc_next (16807, val),
			|val| calc_next (48271, val))
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	#[ inline ]
	fn calc_next (factor: u64, mask: u64, prev: u64) -> u64 {
		let mut cur = prev;
		loop {
			cur = cur.pan_u64 ().wrapping_mul (factor.pan_u64 ()) % 0x_7fff_ffff;
			if cur & mask == 0 { return cur }
		}
	}
	Ok (
		calc_result (
			input,
			input.params.reps_two,
			|val| calc_next (16807, 0x3, val),
			|val| calc_next (48271, 0x7, val))
	)
}

#[ inline ]
fn calc_result (
	input: & Input,
	num_compares: u32,
	calc_a: fn (u64) -> u64,
	calc_b: fn (u64) -> u64,
) -> u32 {
	itertools::iterate ((input.start_a.pan_u64 (), input.start_b.pan_u64 ()),
			|& (a, b)| (calc_a (a), calc_b (b)))
		.skip (1)
		.take (num_compares.pan_usize ())
		.filter (|& (a, b)| a & 0xffff == b & 0xffff)
		.count ()
		.pan_u32 ()
}
