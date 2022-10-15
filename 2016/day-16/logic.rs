use super::*;

use input::Bit;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <String> {
	Ok (calc_result (& input.initial_state, input.params.disk_size_one))
}

pub fn part_two (input: & Input) -> GenResult <String> {
	Ok (calc_result (& input.initial_state, input.params.disk_size_two))
}

fn calc_result (state: & [Bit], disk_size: u32) -> String {

	// iterator over packed integers with bits from sequence

	let mut extend_iter = ExtendIter::new (state);

	// work out checksum size and number of bits they span in the original

	let mut sum_bits = disk_size;
	let mut span_bits = 1_u32;
	while sum_bits & 1 == 0 { sum_bits /= 2; span_bits *= 2; }

	// iterate to build up the result string

	let mut result = String::new ();
	let mut cur = 0;
	let mut cur_bits = 0;
	for _ in 0 .. sum_bits {

		// iterate over the appropriate number of bits

		let mut span_rem = span_bits;
		let mut sum = 0;
		while span_rem > 0 {

			// get another packed word from the iterator

			if cur_bits == 0 {
				cur = extend_iter.next ().unwrap ();
				cur_bits = 64;
			}

			// count the relevant one bits

			if cur_bits <= span_rem {
				sum += cur.count_ones ();
				span_rem -= cur_bits;
				cur_bits = 0;
			} else {
				sum += (cur >> (64 - span_rem)).count_ones ();
				cur <<= span_rem;
				cur_bits -= span_rem;
				break;
			}

		}

		// append the result to the string

		write! (& mut result, "{}", (sum & 1) ^ 1).unwrap ();

	}

	result

}

struct ExtendIter {
	data: u64,
	data_rev: u64,
	data_bits: u32,
	cur: u64,
	cur_bits: u32,
	rev: bool,
	count: usize,
}

impl ExtendIter {

	fn new (data_vec: & [Bit]) -> Self {

		let data_bits = data_vec.len ().pan_u32 ();

		// convert vec to bit pattern

		let mut data = 0_u64;
		for bit in data_vec.iter ().copied () {
			data <<= 1_u32;
			if bit == Bit::One { data |= 1; }
		}

		// also convert to inverted and reversed bit pattern

		let mut data_rev = 0_u64;
		for bit in data_vec.iter ().rev ().copied () {
			data_rev <<= 1_u32;
			if bit == Bit::Zero { data_rev |= 1; }
		}

		// construct

		Self {
			data,
			data_rev,
			data_bits,
			cur: 0,
			cur_bits: 0,
			rev: false,
			count: usize::MAX,
		}

	}

	fn calc_sep_bit (old_count: usize) -> u64 {
		let new_count = old_count + 1;
		let changes = old_count ^ new_count;
		let bit_idx = usize::BITS - changes.leading_zeros ();
		u64::from (new_count & (1 << bit_idx) != 0)
	}

}

impl Iterator for ExtendIter {

	type Item = u64;

	fn next (& mut self) -> Option <u64> {

		// build up packed bits iteratively

		let mut val = 0;
		let mut val_bits = 0;
		while val_bits < 64 {

			// work out next chunk of bits - single 0 or 1, plus data or data_rev

			if self.cur_bits == 0 {
				self.cur = 0;
				if self.count == usize::MAX {
					self.count = 0;
				} else {
					self.cur = Self::calc_sep_bit (self.count);
					self.count += 1;
					self.cur_bits = 1;
				}
				self.cur <<= self.data_bits;
				self.cur |= if self.rev { self.data_rev } else { self.data };
				self.cur_bits += self.data_bits;
				self.rev = ! self.rev;
			}

			// append bits to result

			if val_bits + self.cur_bits <= 64 {
				val |= self.cur << (64 - val_bits - self.cur_bits);
				val_bits += self.cur_bits;
				self.cur_bits = 0;
			} else {
				val |= self.cur >> (self.cur_bits + val_bits - 64);
				self.cur_bits -= 64 - val_bits;
				val_bits = 64;
			}

		}

		// return packed value

		Some (val)

	}

}
