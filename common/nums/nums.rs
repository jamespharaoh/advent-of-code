#![ allow (clippy::as_conversions) ]
#![ allow (clippy::default_numeric_fallback) ]
#![ allow (clippy::inline_always) ]
#![ allow (clippy::wrong_self_convention) ]

use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;
use std::ops::ShrAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::str::FromStr;

pub use crate::bits::*;

pub type NumResult <Val> = Result <Val, Overflow>;

#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
pub struct Overflow;

impl Display for Overflow {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "Overflow") ?;
		Ok (())
	}

}

impl Error for Overflow {
}

pub trait TryAdd <Arg = Self> {
	type Output;
	fn try_add (self, arg: Arg) -> Result <Self::Output, Overflow>;
}

pub trait TryAddAssign <Arg = Self> {
	fn try_add_assign (& mut self, arg: Arg) -> Result <(), Overflow>;
}

pub trait TryDiv <Arg = Self> {
	type Output;
	fn try_div (self, arg: Arg) -> Result <Self::Output, Overflow>;
}

pub trait TryDivAssign <Arg = Self> {
	fn try_div_assign (& mut self, arg: Arg) -> Result <(), Overflow>;
}

pub trait TryMul <Arg = Self> {
	type Output;
	fn try_mul (self, arg: Arg) -> Result <Self::Output, Overflow>;
}

pub trait TryMulAssign <Arg = Self> {
	fn try_mul_assign (self, arg: Arg) -> Result <(), Overflow>;
}

pub trait TryRem <Arg = Self> {
	type Output;
	fn try_rem (self, arg: Arg) -> Result <Self::Output, Overflow>;
}

pub trait TryRemAssign <Arg = Self> {
	fn try_rem_assign (self, arg: Arg) -> Result <(), Overflow>;
}

pub trait TrySub <Arg = Self> {
	type Output;
	fn try_sub (self, arg: Arg) -> Result <Self::Output, Overflow>;
}

pub trait TrySubAssign <Arg = Self> {
	fn try_sub_assign (self, arg: Arg) -> Result <(), Overflow>;
}

pub trait Int: Clone + Copy + Debug + Default + Display + Eq + FromStr + Hash + Ord + IntOps
		+ IntConv {
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

	/// Add two numbers together
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn add_2 (arg_0: Self, arg_1: Self) -> NumResult <Self>;

	/// Add three numbers together
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	#[ inline ]
	fn add_3 (arg_0: Self, arg_1: Self, arg_2: Self) -> NumResult <Self> {
		Self::add_2 (Self::add_2 (arg_0, arg_1) ?, arg_2)
	}

	/// Add four numbers together
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	#[ inline ]
	fn add_4 (arg_0: Self, arg_1: Self, arg_2: Self, arg_3: Self) -> NumResult <Self> {
		Self::add_2 (Self::add_3 (arg_0, arg_1, arg_2) ?, arg_3)
	}

	/// Divide one number by another
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn div_2 (arg_0: Self, arg_1: Self) -> NumResult <Self>;

	/// Multiply two numbers together
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn mul_2 (arg_0: Self, arg_1: Self) -> NumResult <Self>;

	/// Multiply three numbers together
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	#[ inline ]
	fn mul_3 (arg_0: Self, arg_1: Self, arg_2: Self) -> NumResult <Self> {
		Self::mul_2 (Self::mul_2 (arg_0, arg_1) ?, arg_2)
	}

	/// Multiply four numbers together
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	#[ inline ]
	fn mul_4 (arg_0: Self, arg_1: Self, arg_2: Self, arg_3: Self) -> NumResult <Self> {
		Self::mul_2 (Self::mul_3 (arg_0, arg_1, arg_2) ?, arg_3)
	}

	/// Find the remainder from dividing one number by another
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn rem_2 (arg_0: Self, arg_1: Self) -> NumResult <Self>;

	/// Subtract one number from another
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn sub_2 (arg_0: Self, arg_1: Self) -> NumResult <Self>;

	fn gen_count_ones (self) -> u32;

	#[ inline (always) ]
	fn check_bit (self, bit: u32) -> bool {
		self & (Self::ONE << bit) != Self::ZERO
	}

}

pub trait IntOpsTry: Sized +
	TryAdd <Output = Self> +
	TryDiv <Output = Self> +
	TryMul <Output = Self> +
	TryRem <Output = Self> +
	TrySub <Output = Self> {
}

impl <Val> IntOpsTry for Val
	where Val: Sized +
		TryAdd <Output = Self> +
		TryDiv <Output = Self> +
		TryMul <Output = Self> +
		TryRem <Output = Self> +
		TrySub <Output = Self> {
}

macro_rules! prim_int {
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

			#[ inline ]
			fn add_2 (arg_0: $signed, arg_1: $signed) -> NumResult <$signed> {
				$signed::checked_add (arg_0, arg_1).ok_or (Overflow)
			}

			#[ inline ]
			fn div_2 (arg_0: $signed, arg_1: $signed) -> NumResult <$signed> {
				$signed::checked_div (arg_0, arg_1).ok_or (Overflow)
			}

			#[ inline ]
			fn mul_2 (arg_0: $signed, arg_1: $signed) -> NumResult <$signed> {
				$signed::checked_mul (arg_0, arg_1).ok_or (Overflow)
			}

			#[ inline ]
			fn rem_2 (arg_0: $signed, arg_1: $signed) -> NumResult <$signed> {
				$signed::checked_rem (arg_0, arg_1).ok_or (Overflow)
			}

			#[ inline ]
			fn sub_2 (arg_0: $signed, arg_1: $signed) -> NumResult <$signed> {
				$signed::checked_sub (arg_0, arg_1).ok_or (Overflow)
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

			#[ inline ]
			fn add_2 (arg_0: $unsigned, arg_1: $unsigned) -> NumResult <$unsigned> {
				$unsigned::checked_add (arg_0, arg_1).ok_or (Overflow)
			}

			#[ inline ]
			fn div_2 (arg_0: $unsigned, arg_1: $unsigned) -> NumResult <$unsigned> {
				$unsigned::checked_div (arg_0, arg_1).ok_or (Overflow)
			}

			#[ inline ]
			fn mul_2 (arg_0: $unsigned, arg_1: $unsigned) -> NumResult <$unsigned> {
				$unsigned::checked_mul (arg_0, arg_1).ok_or (Overflow)
			}

			#[ inline ]
			fn rem_2 (arg_0: $unsigned, arg_1: $unsigned) -> NumResult <$unsigned> {
				$unsigned::checked_rem (arg_0, arg_1).ok_or (Overflow)
			}

			#[ inline ]
			fn sub_2 (arg_0: $unsigned, arg_1: $unsigned) -> NumResult <$unsigned> {
				$unsigned::checked_sub (arg_0, arg_1).ok_or (Overflow)
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

		impl IntConv for $signed {

			#[ inline (always) ]
			fn from_char (val: char) -> NumResult <Self> {
				val.qck_u32 ().try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn from_u8 (val: u8) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn from_usize (val: usize) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn from_isize (val: isize) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_char (self) -> NumResult <char> {
				self.to_u32 () ?.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i8 (self) -> NumResult <i8> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i16 (self) -> NumResult <i16> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i32 (self) -> NumResult <i32> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i64 (self) -> NumResult <i64> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i128 (self) -> NumResult <i128> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_isize (self) -> NumResult <isize> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u8 (self) -> NumResult <u8> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u16 (self) -> NumResult <u16> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u32 (self) -> NumResult <u32> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u64 (self) -> NumResult <u64> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u128 (self) -> NumResult <u128> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_usize (self) -> NumResult <usize> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn qck_f32 (self) -> f32 { self as f32 }

			#[ inline (always) ]
			fn qck_f64 (self) -> f64 { self as f64 }

			#[ inline (always) ]
			fn qck_i8 (self) -> i8 { self as i8 }

			#[ inline (always) ]
			fn qck_i16 (self) -> i16 { self as i16 }

			#[ inline (always) ]
			fn qck_i32 (self) -> i32 { self as i32 }

			#[ inline (always) ]
			fn qck_i64 (self) -> i64 { self as i64 }

			#[ inline (always) ]
			fn qck_i128 (self) -> i128 { self as i128 }

			#[ inline (always) ]
			fn qck_isize (self) -> isize { self as isize }

			#[ inline (always) ]
			fn qck_u8 (self) -> u8 { self as u8 }

			#[ inline (always) ]
			fn qck_u16 (self) -> u16 { self as u16 }

			#[ inline (always) ]
			fn qck_u32 (self) -> u32 { self as u32 }

			#[ inline (always) ]
			fn qck_u64 (self) -> u64 { self as u64 }

			#[ inline (always) ]
			fn qck_u128 (self) -> u128 { self as u128 }

			#[ inline (always) ]
			fn qck_usize (self) -> usize { self as usize }

		}

		impl IntConv for $unsigned {

			#[ inline (always) ]
			fn from_char (val: char) -> NumResult <Self> {
				val.qck_u32 ().try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn from_u8 (val: u8) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn from_usize (val: usize) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn from_isize (val: isize) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_char (self) -> NumResult <char> {
				self.to_u32 () ?.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i8 (self) -> NumResult <i8> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i16 (self) -> NumResult <i16> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i32 (self) -> NumResult <i32> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i64 (self) -> NumResult <i64> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i128 (self) -> NumResult <i128> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_isize (self) -> NumResult <isize> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u8 (self) -> NumResult <u8> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u16 (self) -> NumResult <u16> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u32 (self) -> NumResult <u32> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u64 (self) -> NumResult <u64> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u128 (self) -> NumResult <u128> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_usize (self) -> NumResult <usize> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn qck_f32 (self) -> f32 { self as f32 }

			#[ inline (always) ]
			fn qck_f64 (self) -> f64 { self as f64 }

			#[ inline (always) ]
			fn qck_i8 (self) -> i8 { self as i8 }

			#[ inline (always) ]
			fn qck_i16 (self) -> i16 { self as i16 }

			#[ inline (always) ]
			fn qck_i32 (self) -> i32 { self as i32 }

			#[ inline (always) ]
			fn qck_i64 (self) -> i64 { self as i64 }

			#[ inline (always) ]
			fn qck_i128 (self) -> i128 { self as i128 }

			#[ inline (always) ]
			fn qck_isize (self) -> isize { self as isize }

			#[ inline (always) ]
			fn qck_u8 (self) -> u8 { self as u8 }

			#[ inline (always) ]
			fn qck_u16 (self) -> u16 { self as u16 }

			#[ inline (always) ]
			fn qck_u32 (self) -> u32 { self as u32 }

			#[ inline (always) ]
			fn qck_u64 (self) -> u64 { self as u64 }

			#[ inline (always) ]
			fn qck_u128 (self) -> u128 { self as u128 }

			#[ inline (always) ]
			fn qck_usize (self) -> usize { self as usize }

		}

		impl TryAdd <Self> for $signed {
			type Output = Self;

			#[ inline (always) ]
			fn try_add (self, arg: Self) -> NumResult <Self> {
				self.checked_add (arg).ok_or (Overflow)
			}

		}

		impl TryAdd <Self> for $unsigned {
			type Output = Self;

			#[ inline (always) ]
			fn try_add (self, arg: Self) -> NumResult <Self> {
				self.checked_add (arg).ok_or (Overflow)
			}

		}

		impl TryAdd <$signed> for $unsigned {
			type Output = Self;

			#[ inline (always) ]
			fn try_add (self, arg: $signed) -> NumResult <Self> {
				self.add_signed (arg)
			}

		}

		impl TryDiv <Self> for $signed {
			type Output = Self;

			#[ inline (always) ]
			fn try_div (self, arg: Self) -> NumResult <Self> {
				self.checked_div (arg).ok_or (Overflow)
			}

		}

		impl TryDiv <Self> for $unsigned {
			type Output = Self;

			#[ inline (always) ]
			fn try_div (self, arg: Self) -> NumResult <Self> {
				self.checked_div (arg).ok_or (Overflow)
			}

		}

		impl TryMul <Self> for $signed {
			type Output = Self;

			#[ inline (always) ]
			fn try_mul (self, arg: Self) -> NumResult <Self> {
				self.checked_mul (arg).ok_or (Overflow)
			}

		}

		impl TryMul <Self> for $unsigned {
			type Output = Self;

			#[ inline (always) ]
			fn try_mul (self, arg: Self) -> NumResult <Self> {
				self.checked_mul (arg).ok_or (Overflow)
			}

		}

		impl TryRem <Self> for $signed {
			type Output = Self;

			#[ inline (always) ]
			fn try_rem (self, arg: Self) -> NumResult <Self> {
				self.checked_rem (arg).ok_or (Overflow)
			}

		}

		impl TryRem <Self> for $unsigned {
			type Output = Self;

			#[ inline (always) ]
			fn try_rem (self, arg: Self) -> NumResult <Self> {
				self.checked_rem (arg).ok_or (Overflow)
			}

		}

		impl TrySub <Self> for $signed {
			type Output = Self;

			#[ inline (always) ]
			fn try_sub (self, arg: Self) -> NumResult <Self> {
				self.checked_sub (arg).ok_or (Overflow)
			}

		}

		impl TrySub <Self> for $unsigned {
			type Output = Self;

			#[ inline (always) ]
			fn try_sub (self, arg: Self) -> NumResult <Self> {
				self.checked_sub (arg).ok_or (Overflow)
			}

		}

		quick_from! ($unsigned, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
		quick_from! ($signed, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

	};

}

macro_rules! quick_from {
	( $target:ident, $( $source:ident ),* ) => {
		$(

			impl QuickFrom <$source> for $target {

				#[ inline (always) ]
				fn quick_from (arg: $source) -> Self {
					arg as $target
				}

			}

		)*
	};
}

prim_int! (i8, u8, 8);
prim_int! (i16, u16, 16);
prim_int! (i32, u32, 32);
prim_int! (i64, u64, 64);
prim_int! (i128, u128, 128);
prim_int! (isize, usize, 128);

pub trait IntSigned: Int + Neg <Output = Self> {
	const NEG_ONE: Self::Signed;
}

pub trait IntUnsigned: Int {}

pub trait IntSized <const BITS: usize>: Int {}

pub trait IntOpsRust: Sized + Add <Output = Self> + AddAssign + BitAnd <Output = Self>
	+ BitAndAssign + BitOr <Output = Self> + BitOrAssign + Div <Output = Self>
	+ Mul <Output = Self> + Rem <Output = Self> + Shl <u32, Output = Self> + ShlAssign <u32>
	+ Shr <u32, Output = Self> + ShrAssign <u32> + Sub <Output = Self> + SubAssign {}
impl <Val> IntOpsRust for Val where Val: Sized + Add <Output = Self> + AddAssign
	+ BitAnd <Output = Self> + BitAndAssign + BitOr <Output = Self> + BitOrAssign
	+ Div <Output = Self> + Mul <Output = Self> + Rem <Output = Self> + Shl <u32, Output = Self>
	+ ShlAssign <u32> + Shr <u32, Output = Self> + ShrAssign <u32> + Sub <Output = Self>
	+ SubAssign {}

pub trait IntOpsSafe: Sized {

	#[ must_use ]
	fn safe_add (self, arg: Self) -> Self;

	#[ must_use ]
	fn safe_sub (self, arg: Self) -> Self;

}

pub trait IntOps: IntOpsRust + IntOpsSafe + IntOpsTry {}
impl <Val> IntOps for Val where Val: IntOpsRust + IntOpsSafe + IntOpsTry {}

pub trait IntConv: Sized {

	#[ inline (always) ]
	fn pan_f32 (self) -> f32 { self.to_f32 ().unwrap () }

	#[ inline (always) ]
	fn pan_f64 (self) -> f64 { self.to_f64 ().unwrap () }

	#[ inline (always) ]
	fn pan_i8 (self) -> i8 { self.to_i8 ().unwrap () }

	#[ inline (always) ]
	fn pan_i16 (self) -> i16 { self.to_i16 ().unwrap () }

	#[ inline (always) ]
	fn pan_i32 (self) -> i32 { self.to_i32 ().unwrap () }

	#[ inline (always) ]
	fn pan_i64 (self) -> i64 { self.to_i64 ().unwrap () }

	#[ inline (always) ]
	fn pan_i128 (self) -> i128 { self.to_i128 ().unwrap () }

	#[ inline (always) ]
	fn pan_isize (self) -> isize { self.to_isize ().unwrap () }

	#[ inline (always) ]
	fn pan_u8 (self) -> u8 { self.to_u8 ().unwrap () }

	#[ inline (always) ]
	fn pan_u16 (self) -> u16 { self.to_u16 ().unwrap () }

	#[ inline (always) ]
	fn pan_u32 (self) -> u32 { self.to_u32 ().unwrap () }

	#[ inline (always) ]
	fn pan_u64 (self) -> u64 { self.to_u64 ().unwrap () }

	#[ inline (always) ]
	fn pan_u128 (self) -> u128 { self.to_u128 ().unwrap () }

	#[ inline (always) ]
	fn pan_usize (self) -> usize { self.to_usize ().unwrap () }

	#[ inline (always) ]
	fn pan_char (self) -> char { self.to_char ().unwrap () }

	fn qck_f32 (self) -> f32;
	fn qck_f64 (self) -> f64;
	fn qck_i8 (self) -> i8;
	fn qck_i16 (self) -> i16;
	fn qck_i32 (self) -> i32;
	fn qck_i64 (self) -> i64;
	fn qck_i128 (self) -> i128;
	fn qck_isize (self) -> isize;
	fn qck_u8 (self) -> u8;
	fn qck_u16 (self) -> u16;
	fn qck_u32 (self) -> u32;
	fn qck_u64 (self) -> u64;
	fn qck_u128 (self) -> u128;
	fn qck_usize (self) -> usize;

	fn from_char (val: char) -> NumResult <Self>;

	fn from_u8 (val: u8) -> NumResult <Self>;

	/// Safely convert from [`usize`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn from_usize (val: usize) -> NumResult <Self>;

	fn from_isize (val: isize) -> NumResult <Self>;

	/// Safely convert to [`f32`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	#[ inline (always) ]
	fn to_f32 (self) -> NumResult <f32> { self.to_u16 ().map (Into::into) }

	/// Safely convert to [`f64`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	#[ inline (always) ]
	fn to_f64 (self) -> NumResult <f64> { self.to_u32 ().map (Into::into) }

	/// Safely convert to [`i8`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_i8 (self) -> NumResult <i8>;

	/// Safely convert to [`i16`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_i16 (self) -> NumResult <i16>;

	/// Safely convert to [`i32`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_i32 (self) -> NumResult <i32>;

	/// Safely convert to [`i64`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_i64 (self) -> NumResult <i64>;

	/// Safely convert to [`i128`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_i128 (self) -> NumResult <i128>;

	/// Safely convert to [`isize`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_isize (self) -> NumResult <isize>;

	/// Safely convert to [`u8`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_u8 (self) -> NumResult <u8>;

	/// Safely convert to [`u16`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_u16 (self) -> NumResult <u16>;

	/// Safely convert to [`u32`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_u32 (self) -> NumResult <u32>;

	/// Safely convert to [`u64`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_u64 (self) -> NumResult <u64>;

	/// Safely convert to [`u128`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_u128 (self) -> NumResult <u128>;

	/// Safely convert to [`usize`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_usize (self) -> NumResult <usize>;

	fn to_char (self) -> NumResult <char>;

}

impl IntConv for char {

	#[ inline (always) ]
	fn from_char (val: char) -> NumResult <Self> {
		Ok (val)
	}

	#[ inline (always) ]
	fn from_u8 (val: u8) -> NumResult <Self> {
		val.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn from_usize (val: usize) -> NumResult <Self> {
		val.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn from_isize (val: isize) -> NumResult <Self> {
		val.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_char (self) -> NumResult <char> {
		Ok (self)
	}

	#[ inline (always) ]
	fn to_i8 (self) -> NumResult <i8> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_i16 (self) -> NumResult <i16> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_i32 (self) -> NumResult <i32> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_i64 (self) -> NumResult <i64> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_i128 (self) -> NumResult <i128> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_isize (self) -> NumResult <isize> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_u8 (self) -> NumResult <u8> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_u16 (self) -> NumResult <u16> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_u32 (self) -> NumResult <u32> {
		self.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always)]
	fn to_u64 (self) -> NumResult <u64> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_u128 (self) -> NumResult <u128> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_usize (self) -> NumResult <usize> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn qck_f32 (self) -> f32 { self.qck_u32 ().qck_f32 () }

	#[ inline (always) ]
	fn qck_f64 (self) -> f64 { self.qck_u32 ().qck_f64 () }

	#[ inline (always) ]
	fn qck_i8 (self) -> i8 { self as i8 }

	#[ inline (always) ]
	fn qck_i16 (self) -> i16 { self as i16 }

	#[ inline (always) ]
	fn qck_i32 (self) -> i32 { self as i32 }

	#[ inline (always) ]
	fn qck_i64 (self) -> i64 { self as i64 }

	#[ inline (always) ]
	fn qck_i128 (self) -> i128 { self as i128 }

	#[ inline (always) ]
	fn qck_isize (self) -> isize { self as isize }

	#[ inline (always) ]
	fn qck_u8 (self) -> u8 { self as u8 }

	#[ inline (always) ]
	fn qck_u16 (self) -> u16 { self as u16 }

	#[ inline (always) ]
	fn qck_u32 (self) -> u32 { self as u32 }

	#[ inline (always) ]
	fn qck_u64 (self) -> u64 { self as u64 }

	#[ inline (always) ]
	fn qck_u128 (self) -> u128 { self as u128 }

	#[ inline (always) ]
	fn qck_usize (self) -> usize { self as usize }

}

pub trait QuickFrom <Other> {
	fn quick_from (other: Other) -> Self;
}

pub trait QuickInto <Other> {
	fn quick_into (self) -> Other;
}

impl <From, To> QuickInto <To> for From where To: QuickFrom <From> {
	#[ inline (always) ]
	fn quick_into (self) -> To {
		To::quick_from (self)
	}
}

mod bits {

	use super::*;

	pub struct BitPusher <Data: Int> {
		data: Data,
		bits: u32,
	}

	impl <Data: Int> BitPusher <Data> {

		#[ inline (always) ]
		#[ must_use ]
		pub fn new () -> Self {
			Self { data: Data::ZERO, bits: 0 }
		}

		#[ inline (always) ]
		pub fn push <Val: Int + QuickInto <Data>> (& mut self, val: Val, bits: u32) {
			debug_assert! (self.bits + bits <= Data::BITS);
			self.data <<= bits;
			self.data |= val.quick_into ();
			self.bits += bits;
		}

		#[ inline (always) ]
		pub fn finish (self) -> Data {
			self.data << (Data::BITS - self.bits)
		}

	}

	impl <Data: Int> Default for BitPusher <Data> {

		#[ inline (always) ]
		fn default () -> Self {
			Self::new ()
		}

	}

	pub struct BitPopper <Data: Int> {
		data: Data,
		bits: u32,
	}

	impl <Data: Int> BitPopper <Data> {

		#[ inline (always) ]
		pub fn new (data: Data) -> Self {
			Self { data, bits: Data::BITS }
		}

		#[ inline (always) ]
		pub fn pop <Val: Int + QuickFrom <Data>> (& mut self, bits: u32) -> Val {
			debug_assert! (bits <= Val::BITS);
			debug_assert! (bits <= self.bits);
			let result = (self.data >> (Data::BITS - bits)).quick_into ();
			self.data <<= bits;
			self.bits -= bits;
			result
		}

	}

}

