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
