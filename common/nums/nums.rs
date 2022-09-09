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
use std::str::FromStr;

pub type NumResult <Val> = Result <Val, Overflow>;

#[ derive (Debug, Eq, PartialEq) ]
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

pub trait Int: Clone + Copy + Debug + Default + Display + Eq + FromStr + Hash + Ord + IntOps + IntConv {
	type Signed: IntSigned;
	type Unsigned: IntUnsigned;
	const BITS: u32;
	const ZERO: Self;
	const ONE: Self;
	const TWO: Self;
	const THREE: Self;
	const FOUR: Self;
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

}

macro_rules! prim_int {
	( $signed:ident , $unsigned:ident, $bits:literal ) => {

		impl Int for $signed {

			type Signed = $signed;
			type Unsigned = $unsigned;

			const BITS: u32 = $signed::BITS;

			#[ allow (clippy::default_numeric_fallback) ]
			const ZERO: $signed = 0;

			#[ allow (clippy::default_numeric_fallback) ]
			const ONE: $signed = 1;

			#[ allow (clippy::default_numeric_fallback) ]
			const TWO: $signed = 2;

			#[ allow (clippy::default_numeric_fallback) ]
			const THREE: $signed = 3;

			#[ allow (clippy::default_numeric_fallback) ]
			const FOUR: $signed = 4;

			const MIN: $signed = $signed::MIN;
			const MAX: $signed = $signed::MAX;

			#[ inline ]
			fn unsigned_abs (self) -> $unsigned { $signed::unsigned_abs (self) }

			#[ inline ]
			fn signum (self) -> $signed { $signed::signum (self) }

			#[ inline ]
			fn signed_diff (self, other: Self) -> NumResult <$signed> {
				$signed::checked_sub (self, other).ok_or (Overflow)
			}

			#[ inline ]
			fn unsigned_diff (self, other: Self) -> NumResult <$unsigned> {
				(other <= self).then (|| $signed::abs_diff (self, other)).ok_or (Overflow)
			}

			#[ inline ]
			fn add_signed (self, other: $signed) -> NumResult <$signed> {
				$signed::checked_add (self, other).ok_or (Overflow)
			}

			#[ inline ]
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
			const MIN: $unsigned = $unsigned::MIN;
			const MAX: $unsigned = $unsigned::MAX;

			#[ inline ]
			fn unsigned_abs (self) -> $unsigned { self }

			#[ inline ]
			fn signum (self) -> $signed {
				if self > 0 { Self::Signed::ONE } else { Self::Signed::ZERO }
			}

			#[ inline ]
			fn signed_diff (self, other: Self) -> NumResult <$signed> {
				if other < self {
					(self - other).try_into ().ok ().ok_or (Overflow)
				} else {
					(other - self).try_into ().map ($signed::neg).ok ().ok_or (Overflow)
				}
			}

			#[ inline ]
			fn unsigned_diff (self, other: Self) -> NumResult <$unsigned> {
				$unsigned::checked_sub (self, other).ok_or (Overflow)
			}

			#[ inline ]
			fn add_signed (self, other: $signed) -> NumResult <$unsigned> {
				if other >= Self::Signed::ZERO {
					$unsigned::checked_add (self, $signed::unsigned_abs (other)).ok_or (Overflow)
				} else {
					$unsigned::checked_sub (self, $signed::unsigned_abs (other)).ok_or (Overflow)
				}
			}

			#[ inline ]
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

		}

		impl IntOpsSafe for $signed {

			#[ inline ]
			fn safe_add (self, arg: $signed) -> $signed {
				$signed::checked_add (self, arg).unwrap ()
			}

			#[ inline ]
			fn safe_sub (self, arg: $signed) -> $signed {
				$signed::checked_sub (self, arg).unwrap ()
			}

		}

		impl IntOpsSafe for $unsigned {

			#[ inline ]
			fn safe_add (self, arg: $unsigned) -> $unsigned {
				$unsigned::checked_add (self, arg).unwrap ()
			}

			#[ inline ]
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

			#[ inline ]
			fn from_char (val: char) -> NumResult <Self> {
				val.as_u32 ().try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn from_u8 (val: u8) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn from_usize (val: usize) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn from_isize (val: isize) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_char (self) -> NumResult <char> {
				self.to_u32 () ?.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_i8 (self) -> NumResult <i8> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_i16 (self) -> NumResult <i16> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_i32 (self) -> NumResult <i32> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_i64 (self) -> NumResult <i64> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_i128 (self) -> NumResult <i128> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_isize (self) -> NumResult <isize> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_u8 (self) -> NumResult <u8> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_u16 (self) -> NumResult <u16> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_u32 (self) -> NumResult <u32> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_u64 (self) -> NumResult <u64> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_u128 (self) -> NumResult <u128> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_usize (self) -> NumResult <usize> {
				self.try_into ().ok ().ok_or (Overflow)
			}

		}

		impl IntConv for $unsigned {

			#[ inline ]
			fn from_char (val: char) -> NumResult <Self> {
				val.as_u32 ().try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn from_u8 (val: u8) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn from_usize (val: usize) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn from_isize (val: isize) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_char (self) -> NumResult <char> {
				self.to_u32 () ?.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_i8 (self) -> NumResult <i8> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_i16 (self) -> NumResult <i16> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_i32 (self) -> NumResult <i32> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_i64 (self) -> NumResult <i64> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_i128 (self) -> NumResult <i128> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_isize (self) -> NumResult <isize> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_u8 (self) -> NumResult <u8> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_u16 (self) -> NumResult <u16> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_u32 (self) -> NumResult <u32> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_u64 (self) -> NumResult <u64> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_u128 (self) -> NumResult <u128> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline ]
			fn to_usize (self) -> NumResult <usize> {
				self.try_into ().ok ().ok_or (Overflow)
			}

		}

	};

}

prim_int! (i8, u8, 8);
prim_int! (i16, u16, 16);
prim_int! (i32, u32, 32);
prim_int! (i64, u64, 64);
prim_int! (i128, u128, 128);
prim_int! (isize, usize, 128);

pub trait IntSigned: Int {
	const NEG_ONE: Self::Signed;
}

pub trait IntUnsigned: Int {}

pub trait IntSized <const BITS: usize>: Int {}

pub trait IntOpsRust: Sized + Add <Output = Self> + AddAssign + BitAnd <Output = Self>
	+ BitAndAssign + BitOr <Output = Self> + BitOrAssign + Div <Output = Self>
	+ Mul <Output = Self> + Rem <Output = Self> + Shl <u32, Output = Self> + ShlAssign <u32>
	+ Shr <u32, Output = Self> + ShrAssign <u32> + Sub <Output = Self> {}
impl <Val> IntOpsRust for Val where Val: Sized + Add <Output = Self> + AddAssign
	+ BitAnd <Output = Self> + BitAndAssign + BitOr <Output = Self> + BitOrAssign
	+ Div <Output = Self> + Mul <Output = Self> + Rem <Output = Self> + Shl <u32, Output = Self>
	+ ShlAssign <u32> + Shr <u32, Output = Self> + ShrAssign <u32> + Sub <Output = Self> {}

pub trait IntOpsSafe: Sized {

	#[ must_use ]
	fn safe_add (self, arg: Self) -> Self;

	#[ must_use ]
	fn safe_sub (self, arg: Self) -> Self;

}

pub trait IntOps: IntOpsRust + IntOpsSafe {}
impl <Val> IntOps for Val where Val: IntOpsRust + IntOpsSafe {}

pub trait IntConv: Sized {

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_f32 (self) -> f32 { self.to_f32 ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_f64 (self) -> f64 { self.to_f64 ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_i8 (self) -> i8 { self.to_i8 ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_i16 (self) -> i16 { self.to_i16 ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_i32 (self) -> i32 { self.to_i32 ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_i64 (self) -> i64 { self.to_i64 ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_i128 (self) -> i128 { self.to_i128 ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_isize (self) -> isize { self.to_isize ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_u8 (self) -> u8 { self.to_u8 ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_u16 (self) -> u16 { self.to_u16 ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_u32 (self) -> u32 { self.to_u32 ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_u64 (self) -> u64 { self.to_u64 ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_u128 (self) -> u128 { self.to_u128 ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_usize (self) -> usize { self.to_usize ().unwrap () }

	#[ allow (clippy::wrong_self_convention) ]
	#[ inline ]
	fn as_char (self) -> char { self.to_char ().unwrap () }

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
	#[ inline ]
	fn to_f32 (self) -> NumResult <f32> { self.to_u16 ().map (Into::into) }

	/// Safely convert to [`f64`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	#[ inline ]
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

	#[ inline ]
	fn from_char (val: char) -> NumResult <Self> {
		Ok (val)
	}

	#[ inline ]
	fn from_u8 (val: u8) -> NumResult <Self> {
		val.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn from_usize (val: usize) -> NumResult <Self> {
		val.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn from_isize (val: isize) -> NumResult <Self> {
		val.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn to_char (self) -> NumResult <char> {
		Ok (self)
	}

	#[ inline ]
	fn to_i8 (self) -> NumResult <i8> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn to_i16 (self) -> NumResult <i16> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn to_i32 (self) -> NumResult <i32> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn to_i64 (self) -> NumResult <i64> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn to_i128 (self) -> NumResult <i128> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn to_isize (self) -> NumResult <isize> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn to_u8 (self) -> NumResult <u8> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn to_u16 (self) -> NumResult <u16> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn to_u32 (self) -> NumResult <u32> {
		self.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn to_u64 (self) -> NumResult <u64> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn to_u128 (self) -> NumResult <u128> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline ]
	fn to_usize (self) -> NumResult <usize> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

}
