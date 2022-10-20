#![ allow (clippy::as_conversions) ]
#![ allow (clippy::default_numeric_fallback) ]
#![ allow (clippy::inline_always) ]
#![ allow (clippy::wrong_self_convention) ]

use std::cmp;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

mod bits;
mod conv;
mod int;
mod iter;
mod ops;

pub use crate::bits::*;
pub use crate::conv::*;
pub use crate::int::*;
pub use crate::iter::*;
pub use crate::ops::*;

pub mod prelude {
	pub use crate::{ BitPopper, BitPusher };
	pub use crate::Int;
	pub use crate::IntConv;
	pub use crate::{ IntSigned, IntUnsigned };
	pub use crate::IteratorNums;
	pub use crate::{ NumResult, Overflow };
	pub use crate::{ QuickFrom, QuickInto };
	pub use crate::{ TryAdd, TryAddAssign };
	pub use crate::{ TryDiv, TryDivAssign };
	pub use crate::{ TryMul, TryMulAssign };
	pub use crate::{ TryRem, TryRemAssign };
	pub use crate::{ TrySub, TrySubAssign };
}

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
