use super::*;
use std::fmt::{ Debug, Display };

pub type NumResult <Val> = Result <Val, Overflow>;

#[ derive (Debug, Eq, PartialEq) ]
pub struct Overflow;

impl Display for Overflow {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "Overflow") ?;
		Ok (())
	}
}

impl Error for Overflow {
}

pub trait Int: Clone + Copy + Debug + Display + Eq + Hash + Ord + IntOps + IntConv {
	type Signed: IntSigned;
	type Unsigned: IntUnsigned;
	const ZERO: Self;
	const ONE: Self;
	const MIN: Self;
	const MAX: Self;
	fn as_signed (self) -> Self::Signed;
	fn as_unsigned (self) -> Self::Unsigned;
	fn unsigned_abs (self) -> Self::Unsigned;
	fn signum (self) -> Self::Signed;
	fn signed_diff (self, other: Self) -> NumResult <Self::Signed>;
	fn unsigned_diff (self, other: Self) -> NumResult <Self::Unsigned>;
	fn add_signed (self, other: Self::Signed) -> NumResult <Self>;
	fn sub_signed (self, other: Self::Signed) -> NumResult <Self>;
	fn add_2 (arg_0: Self, arg_1: Self) -> NumResult <Self>;
	#[ inline ]
	fn add_3 (arg_0: Self, arg_1: Self, arg_2: Self) -> NumResult <Self> {
		Self::add_2 (Self::add_2 (arg_0, arg_1) ?, arg_2)
	}
	#[ inline ]
	fn add_4 (arg_0: Self, arg_1: Self, arg_2: Self, arg_3: Self) -> NumResult <Self> {
		Self::add_2 (Self::add_3 (arg_0, arg_1, arg_2) ?, arg_3)
	}
	fn mul_2 (arg_0: Self, arg_1: Self) -> NumResult <Self>;
	#[ inline ]
	fn mul_3 (arg_0: Self, arg_1: Self, arg_2: Self) -> NumResult <Self> {
		Self::mul_2 (Self::mul_2 (arg_0, arg_1) ?, arg_2)
	}
	#[ inline ]
	fn mul_4 (arg_0: Self, arg_1: Self, arg_2: Self, arg_3: Self) -> NumResult <Self> {
		Self::mul_2 (Self::mul_3 (arg_0, arg_1, arg_2) ?, arg_3)
	}
	fn sub_2 (arg_0: Self, arg_1: Self) -> NumResult <Self>;
}

macro_rules! prim_int {
	( $signed:ident , $unsigned:ident, $bits:literal ) => {
		impl Int for $signed {
			type Signed = $signed;
			type Unsigned = $unsigned;
			const ZERO: $signed = 0;
			const ONE: $signed = 1;
			const MIN: $signed = $signed::MIN;
			const MAX: $signed = $signed::MAX;
			#[ inline ]
			fn as_signed (self) -> $signed { self }
			#[ inline ]
			fn as_unsigned (self) -> $unsigned { self as $unsigned }
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
			fn mul_2 (arg_0: $signed, arg_1: $signed) -> NumResult <$signed> {
				$signed::checked_mul (arg_0, arg_1).ok_or (Overflow)
			}
			#[ inline ]
			fn sub_2 (arg_0: $signed, arg_1: $signed) -> NumResult <$signed> {
				$signed::checked_sub (arg_0, arg_1).ok_or (Overflow)
			}
		}
		impl Int for $unsigned {
			type Signed = $signed;
			type Unsigned = $unsigned;
			const ZERO: $unsigned = 0;
			const ONE: $unsigned = 1;
			const MIN: $unsigned = $unsigned::MIN;
			const MAX: $unsigned = $unsigned::MAX;
			#[ inline ]
			fn as_signed (self) -> $signed { self as $signed }
			#[ inline ]
			fn as_unsigned (self) -> $unsigned { self }
			#[ inline ]
			fn unsigned_abs (self) -> $unsigned { self }
			#[ inline ]
			fn signum (self) -> $signed {
				if self > 0 { 1 } else { 0 }
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
				if other >= 0 {
					$unsigned::checked_add (self, other as $unsigned).ok_or (Overflow)
				} else {
					$unsigned::checked_sub (self, $signed::unsigned_abs (other)).ok_or (Overflow)
				}
			}
			#[ inline ]
			fn sub_signed (self, other: $signed) -> NumResult <$unsigned> {
				if other >= 0 {
					$unsigned::checked_sub (self, other as $unsigned).ok_or (Overflow)
				} else {
					$unsigned::checked_add (self, $signed::unsigned_abs (other)).ok_or (Overflow)
				}
			}
			#[ inline ]
			fn add_2 (arg_0: $unsigned, arg_1: $unsigned) -> NumResult <$unsigned> {
				$unsigned::checked_add (arg_0, arg_1).ok_or (Overflow)
			}
			#[ inline ]
			fn mul_2 (arg_0: $unsigned, arg_1: $unsigned) -> NumResult <$unsigned> {
				$unsigned::checked_mul (arg_0, arg_1).ok_or (Overflow)
			}
			#[ inline ]
			fn sub_2 (arg_0: $unsigned, arg_1: $unsigned) -> NumResult <$unsigned> {
				$unsigned::checked_sub (arg_0, arg_1).ok_or (Overflow)
			}
		}
		impl IntOpsSafe for $signed {
			fn safe_add (self, arg: $signed) -> $signed {
				$signed::checked_add (self, arg).unwrap ()
			}
			fn safe_sub (self, arg: $signed) -> $signed {
				$signed::checked_sub (self, arg).unwrap ()
			}
		}
		impl IntOpsSafe for $unsigned {
			fn safe_add (self, arg: $unsigned) -> $unsigned {
				$unsigned::checked_add (self, arg).unwrap ()
			}
			fn safe_sub (self, arg: $unsigned) -> $unsigned {
				$unsigned::checked_sub (self, arg).unwrap ()
			}
		}
		impl IntSigned for $signed {
			const NEG_ONE: $signed = -1;
		}
		impl IntUnsigned for $unsigned {}
		impl IntSized <$bits> for $signed {}
		impl IntSized <$bits> for $unsigned {}
		impl IntConv for $signed {
			fn to_usize (self) -> NumResult <usize> { self.try_into ().ok ().ok_or (Overflow) }
			fn to_u32 (self) -> NumResult <u32> { self.try_into ().ok ().ok_or (Overflow) }
			fn from_usize (val: usize) -> NumResult <Self> { val.try_into ().ok ().ok_or (Overflow) }
		}
		impl IntConv for $unsigned {
			fn to_usize (self) -> NumResult <usize> { self.try_into ().ok ().ok_or (Overflow) }
			fn to_u32 (self) -> NumResult <u32> { self.try_into ().ok ().ok_or (Overflow) }
			fn from_usize (val: usize) -> NumResult <Self> { val.try_into ().ok ().ok_or (Overflow) }
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

pub trait IntOpsRust: Sized
	+ Add <Output = Self> + Div <Output = Self> + Mul <Output = Self> + Rem <Output = Self>
	+ Sub <Output = Self> {}
impl <Val> IntOpsRust for Val where Val: Sized
	+ Add <Output = Self> + Div <Output = Self> + Mul <Output = Self> + Rem <Output = Self>
	+ Sub <Output = Self> {}

pub trait IntOpsSafe: Sized {
	fn safe_add (self, arg: Self) -> Self;
	fn safe_sub (self, arg: Self) -> Self;
}

pub trait IntOps: IntOpsRust + IntOpsSafe {}
impl <Val> IntOps for Val where Val: IntOpsRust + IntOpsSafe {}

pub trait IntConv: Sized {
	fn to_usize (self) -> NumResult <usize>;
	fn to_u32 (self) -> NumResult <u32>;
	fn from_usize (val: usize) -> NumResult <Self>;
}
