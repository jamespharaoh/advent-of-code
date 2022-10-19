use super::*;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Not;
use std::ops::Rem;
use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;
use std::ops::ShrAssign;
use std::ops::Sub;
use std::ops::SubAssign;

pub trait IntOps: IntOpsRust + IntOpsSafe + IntOpsTry {
}

impl <Val> IntOps for Val where Val: IntOpsRust + IntOpsSafe + IntOpsTry {
}

pub trait IntOpsRust: Sized + Add <Output = Self> + AddAssign + BitAnd <Output = Self>
	+ BitAndAssign + BitOr <Output = Self> + BitOrAssign + Div <Output = Self>
	+ Mul <Output = Self> + Not <Output = Self> + Rem <Output = Self> + Shl <u32, Output = Self>
	+ ShlAssign <u32> + Shr <u32, Output = Self> + ShrAssign <u32> + Sub <Output = Self>
	+ SubAssign {
}

impl <Val> IntOpsRust for Val where Val: Sized + Add <Output = Self> + AddAssign
	+ BitAnd <Output = Self> + BitAndAssign + BitOr <Output = Self> + BitOrAssign
	+ Div <Output = Self> + Mul <Output = Self> + Not <Output = Self> + Rem <Output = Self>
	+ Shl <u32, Output = Self> + ShlAssign <u32> + Shr <u32, Output = Self> + ShrAssign <u32>
	+ Sub <Output = Self> + SubAssign {
}

pub trait IntOpsSafe: Sized {

	#[ must_use ]
	fn safe_add (self, arg: Self) -> Self;

	#[ must_use ]
	fn safe_sub (self, arg: Self) -> Self;

}

pub trait IntOpsTry: Sized +
	TryAdd <Output = Self> + TryAddAssign +
	TryDiv <Output = Self> + TryDivAssign +
	TryMul <Output = Self> + TryMulAssign +
	TryRem <Output = Self> + TryRemAssign +
	TrySub <Output = Self> + TrySubAssign {
}

impl <Val> IntOpsTry for Val
	where Val: Sized +
		TryAdd <Output = Self> + TryAddAssign +
		TryDiv <Output = Self> + TryDivAssign +
		TryMul <Output = Self> + TryMulAssign +
		TryRem <Output = Self> + TryRemAssign +
		TrySub <Output = Self> + TrySubAssign {
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
	fn try_mul_assign (& mut self, arg: Arg) -> Result <(), Overflow>;
}

pub trait TryRem <Arg = Self> {
	type Output;
	fn try_rem (self, arg: Arg) -> Result <Self::Output, Overflow>;
}

pub trait TryRemAssign <Arg = Self> {
	fn try_rem_assign (& mut self, arg: Arg) -> Result <(), Overflow>;
}

pub trait TrySub <Arg = Self> {
	type Output;
	fn try_sub (self, arg: Arg) -> Result <Self::Output, Overflow>;
}

pub trait TrySubAssign <Arg = Self> {
	fn try_sub_assign (& mut self, arg: Arg) -> Result <(), Overflow>;
}

macro_rules! int_ops_impl {
	( $signed:ident , $unsigned:ident, $bits:literal ) => {

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

		impl TryAddAssign <Self> for $signed {

			#[ inline (always) ]
			fn try_add_assign (& mut self, arg: Self) -> NumResult <()> {
				* self = self.checked_add (arg).ok_or (Overflow) ?;
				Ok (())
			}

		}

		impl TryAddAssign <Self> for $unsigned {

			#[ inline (always) ]
			fn try_add_assign (& mut self, arg: Self) -> NumResult <()> {
				* self = self.checked_add (arg).ok_or (Overflow) ?;
				Ok (())
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

		impl TryDivAssign <Self> for $signed {

			#[ inline (always) ]
			fn try_div_assign (& mut self, arg: Self) -> NumResult <()> {
				* self = self.checked_div (arg).ok_or (Overflow) ?;
				Ok (())
			}

		}

		impl TryDivAssign <Self> for $unsigned {

			#[ inline (always) ]
			fn try_div_assign (& mut self, arg: Self) -> NumResult <()> {
				* self = self.checked_div (arg).ok_or (Overflow) ?;
				Ok (())
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

		impl TryMulAssign <Self> for $signed {

			#[ inline (always) ]
			fn try_mul_assign (& mut self, arg: Self) -> NumResult <()> {
				* self = self.checked_mul (arg).ok_or (Overflow) ?;
				Ok (())
			}

		}

		impl TryMulAssign <Self> for $unsigned {

			#[ inline (always) ]
			fn try_mul_assign (& mut self, arg: Self) -> NumResult <()> {
				* self = self.checked_mul (arg).ok_or (Overflow) ?;
				Ok (())
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

		impl TryRemAssign <Self> for $signed {

			#[ inline (always) ]
			fn try_rem_assign (& mut self, arg: Self) -> NumResult <()> {
				* self = self.checked_rem (arg).ok_or (Overflow) ?;
				Ok (())
			}

		}

		impl TryRemAssign <Self> for $unsigned {

			#[ inline (always) ]
			fn try_rem_assign (& mut self, arg: Self) -> NumResult <()> {
				* self = self.checked_rem (arg).ok_or (Overflow) ?;
				Ok (())
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

		impl TrySubAssign <Self> for $signed {

			#[ inline (always) ]
			fn try_sub_assign (& mut self, arg: Self) -> NumResult <()> {
				* self = self.checked_sub (arg).ok_or (Overflow) ?;
				Ok (())
			}

		}

		impl TrySubAssign <Self> for $unsigned {

			#[ inline (always) ]
			fn try_sub_assign (& mut self, arg: Self) -> NumResult <()> {
				* self = self.checked_sub (arg).ok_or (Overflow) ?;
				Ok (())
			}

		}

	};
}

int_ops_impl! (i8, u8, 8);
int_ops_impl! (i16, u16, 16);
int_ops_impl! (i32, u32, 32);
int_ops_impl! (i64, u64, 64);
int_ops_impl! (i128, u128, 128);
int_ops_impl! (isize, usize, 128);

