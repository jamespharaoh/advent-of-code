//! Advent of Code 2016: Day 16: Dragon Checksum
//!
//! [https://adventofcode.com/2016/day/16](https://adventofcode.com/2016/day/16)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Dragon Checksum";
	year = 2016;
	day = 16;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Bit;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <String> {
		Ok (calc_result (& input.initial_state, input.disk_size_one))
	}

	pub fn part_two (input: & Input) -> GenResult <String> {
		Ok (calc_result (& input.initial_state, input.disk_size_two))
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

			let data_bits = data_vec.len ().as_u32 ();

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

		const fn calc_sep_bit (old_count: usize) -> u64 {
			let new_count = old_count + 1;
			let changes = old_count ^ new_count;
			let bit_idx = usize::BITS - changes.leading_zeros ();
			if new_count & (1 << bit_idx) != 0 { 1 } else { 0 }
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

}

pub mod model {

	use super::*;

	#[ derive (Clone, Debug, Default, Eq, PartialEq) ]
	pub struct Input {
		pub initial_state: Vec <Bit>,
		pub disk_size_one: u32,
		pub disk_size_two: u32,
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Bit { Zero, One }

	impl Input {

		pub fn parse (mut input: & [& str]) -> GenResult <Self> {

			let disk_size_one = parser::input_param (& mut input, "DISK_SIZE_ONE=", 272_u32) ?;
			if disk_size_one < 1 { return Err ("Disk size one must be at least 1".into ()) };

			let disk_size_two = parser::input_param (& mut input, "DISK_SIZE_TWO=", 35_651_584_u32) ?;
			if disk_size_two < 1 { return Err ("Disk size two must be at least 1".into ()) };

			if input.len () != 1 { return Err ("Input must have exactly one line".into ()) }

			let initial_state: Vec <Bit> =
				input [0].chars ()
					.map (|ch| match ch {
						'0' => Ok (Bit::Zero),
						'1' => Ok (Bit::One),
						_ => Err (format! ("Invalid character: {}", ch).into ()),
					})
				.collect::<GenResult <_>> () ?;
			if initial_state.is_empty () { return Err ("Min initial state size is 1 bit".into ()) }
			if initial_state.len () > 50 { return Err ("Max initial state size is 50 bits".into ()) }

			Ok (Self { initial_state, disk_size_one, disk_size_two })

		}

	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"DISK_SIZE_ONE=20",
		"DISK_SIZE_TWO=80",
		"10000",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("01100", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("11010", puzzle.part_two (EXAMPLE));
	}

}
