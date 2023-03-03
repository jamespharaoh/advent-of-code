use super::*;

use std::ops::Bound;
use std::ops::Neg;
use std::ops::RangeBounds;

pub trait Int: Clone + Copy + Debug + Default + Display + Eq + FromStr + Hash + Ord
		+ IntConv + IntOps {
	type Signed: IntSigned;
	type Unsigned: IntUnsigned;
	const BITS: u32;
	const ZERO: Self;
	const ONE: Self;
	const TWO: Self;
	const THREE: Self;
	const FOUR: Self;
	const FIVE: Self;
	const SIX: Self;
	const SEVEN: Self;
	const EIGHT: Self;
	const NINE: Self;
	const MIN: Self;
	const MAX: Self;
	fn unsigned_abs (self) -> Self::Unsigned;
	fn signum (self) -> Self::Signed;

	/// Signed difference between two numbers
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented, ie if the difference is too
	/// high
	///
	fn signed_diff (self, other: Self) -> NumResult <Self::Signed>;

	/// Unsigned difference between two numbers
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented, ie if the second number is
	/// greater than the first
	///
	fn unsigned_diff (self, other: Self) -> NumResult <Self::Unsigned>;

	/// Add a signed number
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn add_signed (self, other: Self::Signed) -> NumResult <Self>;

	/// Subtract a signed number
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn sub_signed (self, other: Self::Signed) -> NumResult <Self>;

	/// Generic wrapper for primitive `count_ones` function.
	///
	fn gen_count_ones (self) -> u32;

	#[ inline (always) ]
	#[ must_use ]
	fn check_bit (self, bit: u32) -> bool {
		self & (Self::ONE << bit) != Self::ZERO
	}

	#[ inline (always) ]
	#[ must_use ]
	fn bit_set (self, bit: u32) -> Self {
		self | Self::ONE << bit
	}

	#[ inline (always) ]
	fn bit_set_assign (& mut self, bit: u32) {
		* self |= Self::ONE << bit;
	}

	#[ inline (always) ]
	fn bit_clear_assign (& mut self, bit: u32) {
		* self &= ! (Self::ONE << bit);
	}

	#[ inline (always) ]
	fn bound_start_assign (& mut self, other: Self) {
		* self = cmp::max (* self, other);
	}

	#[ inline (always) ]
	fn bound_end_assign (& mut self, other: Self) {
		* self = cmp::min (* self, other);
	}

	#[ inline (always) ]
	fn bounds_assign (& mut self, bounds: impl RangeBounds <Self>) {
		let start = match bounds.start_bound () {
			Bound::Included (& bound) => Some (bound),
			Bound::Excluded (& bound) => Some (bound + Self::ONE),
			Bound::Unbounded => None,
		};
		let end = match bounds.end_bound () {
			Bound::Included (& bound) => Some (bound),
			Bound::Excluded (& bound) => Some (bound - Self::ONE),
			Bound::Unbounded => None,
		};
		if let (Some (start), Some (end)) = (start, end) {
			assert! (start <= end);
		}
		if let Some (start) = start { self.bound_start_assign (start); }
		if let Some (end) = end { self.bound_end_assign (end); }
	}

	#[ inline (always) ]
	fn lcm (num_0: Self, num_1: Self) -> Self {
		let gcd = Self::gcd (num_0, num_1);
		num_0 * num_1 / gcd
	}

	#[ inline (always) ]
	fn gcd (mut num_0: Self, mut num_1: Self) -> Self {
		loop {
			let rem = num_0 % num_1;
			if rem == Self::ZERO { return num_1 }
			num_0 = num_1;
			num_1 = rem;
		}
	}

}

macro_rules! int_impl {
	( $signed:ident , $unsigned:ident, $bits:literal ) => {

		impl Int for $signed {

			type Signed = $signed;
			type Unsigned = $unsigned;

			const BITS: u32 = $signed::BITS;

			const ZERO: $signed = 0;
			const ONE: $signed = 1;
			const TWO: $signed = 2;
			const THREE: $signed = 3;
			const FOUR: $signed = 4;
			const FIVE: $signed = 5;
			const SIX: $signed = 6;
			const SEVEN: $signed = 7;
			const EIGHT: $signed = 8;
			const NINE: $signed = 9;

			const MIN: $signed = $signed::MIN;
			const MAX: $signed = $signed::MAX;

			#[ inline (always) ]
			fn unsigned_abs (self) -> $unsigned { $signed::unsigned_abs (self) }

			#[ inline (always) ]
			fn signum (self) -> $signed { $signed::signum (self) }

			#[ inline (always) ]
			fn signed_diff (self, other: Self) -> NumResult <$signed> {
				$signed::checked_sub (self, other).ok_or (Overflow)
			}

			#[ inline (always) ]
			fn unsigned_diff (self, other: Self) -> NumResult <$unsigned> {
				(other <= self).then (|| $signed::abs_diff (self, other)).ok_or (Overflow)
			}

			#[ inline (always) ]
			fn add_signed (self, other: $signed) -> NumResult <$signed> {
				$signed::checked_add (self, other).ok_or (Overflow)
			}

			#[ inline (always) ]
			fn sub_signed (self, other: $signed) -> NumResult <$signed> {
				$signed::checked_sub (self, other).ok_or (Overflow)
			}

			#[ inline (always) ]
			fn gen_count_ones (self) -> u32 {
				self.count_ones ()
			}

		}

		impl Int for $unsigned {

			type Signed = $signed;
			type Unsigned = $unsigned;

			const BITS: u32 = $signed::BITS;
			const ZERO: $unsigned = 0;
			const ONE: $unsigned = 1;
			const TWO: $unsigned = 2;
			const THREE: $unsigned = 3;
			const FOUR: $unsigned = 4;
			const FIVE: $unsigned = 5;
			const SIX: $unsigned = 6;
			const SEVEN: $unsigned = 7;
			const EIGHT: $unsigned = 8;
			const NINE: $unsigned = 9;
			const MIN: $unsigned = $unsigned::MIN;
			const MAX: $unsigned = $unsigned::MAX;

			#[ inline (always) ]
			fn unsigned_abs (self) -> $unsigned { self }

			#[ inline (always) ]
			fn signum (self) -> $signed {
				if self > 0 { Self::Signed::ONE } else { Self::Signed::ZERO }
			}

			#[ inline (always) ]
			fn signed_diff (self, other: Self) -> NumResult <$signed> {
				if other < self {
					(self - other).try_into ().ok ().ok_or (Overflow)
				} else {
					(other - self).try_into ().map ($signed::neg).ok ().ok_or (Overflow)
				}
			}

			#[ inline (always) ]
			fn unsigned_diff (self, other: Self) -> NumResult <$unsigned> {
				$unsigned::checked_sub (self, other).ok_or (Overflow)
			}

			#[ inline (always) ]
			fn add_signed (self, other: $signed) -> NumResult <$unsigned> {
				if other >= Self::Signed::ZERO {
					$unsigned::checked_add (self, $signed::unsigned_abs (other)).ok_or (Overflow)
				} else {
					$unsigned::checked_sub (self, $signed::unsigned_abs (other)).ok_or (Overflow)
				}
			}

			#[ inline (always) ]
			fn sub_signed (self, other: $signed) -> NumResult <$unsigned> {
				if other >= Self::Signed::ZERO {
					$unsigned::checked_sub (self, $signed::unsigned_abs (other)).ok_or (Overflow)
				} else {
					$unsigned::checked_add (self, $signed::unsigned_abs (other)).ok_or (Overflow)
				}
			}

			#[ inline (always) ]
			fn gen_count_ones (self) -> u32 {
				self.count_ones ()
			}

		}

		impl IntOpsSafe for $signed {

			#[ inline (always) ]
			fn safe_add (self, arg: $signed) -> $signed {
				$signed::checked_add (self, arg).unwrap ()
			}

			#[ inline (always) ]
			fn safe_sub (self, arg: $signed) -> $signed {
				$signed::checked_sub (self, arg).unwrap ()
			}

		}

		impl IntOpsSafe for $unsigned {

			#[ inline (always) ]
			fn safe_add (self, arg: $unsigned) -> $unsigned {
				$unsigned::checked_add (self, arg).unwrap ()
			}

			#[ inline (always) ]
			fn safe_sub (self, arg: $unsigned) -> $unsigned {
				$unsigned::checked_sub (self, arg).unwrap ()
			}

		}

		impl IntSigned for $signed {
			const NEG_ONE: $signed = Self::ZERO - Self::ONE;
		}

		impl IntUnsigned for $unsigned {}

		impl IntSized <$bits> for $signed {}
		impl IntSized <$bits> for $unsigned {}

	};

}

int_impl! (i8, u8, 8);
int_impl! (i16, u16, 16);
int_impl! (i32, u32, 32);
int_impl! (i64, u64, 64);
int_impl! (i128, u128, 128);
int_impl! (isize, usize, 128);

pub trait IntSigned: Int + Neg <Output = Self> {
	const NEG_ONE: Self::Signed;
}

pub trait IntUnsigned: Int {}

pub trait IntSized <const BITS: usize>: Int {}
