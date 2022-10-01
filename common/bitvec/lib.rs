//! Dynamically sized array of items encoded as bits and packed

use std::cmp;
use std::marker::PhantomData;

use aoc_nums::IntConv as _;

mod bitvec;
mod encode;
mod iter;

pub use bitvec::*;
pub use encode::*;
pub use iter::*;

pub mod prelude {
	pub use super::BitVec;
	pub use super::BitVecEncoding;
	pub use super::BitVecNative;
}
