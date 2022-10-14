//! Common functionality used in all puzzle solutions

use clap::ArgMatches;
use clap::Command;

#[ doc (no_inline) ]
pub use prelude::*;

pub mod puzzle;

mod prelude {

	pub use clap;

	pub use aoc_checked::checked as chk;
	pub use aoc_misc::prelude::*;
	pub use aoc_nums as nums;
	pub use aoc_inpstr::InpStr;
	pub use aoc_parser as parser;

	pub use crate::nums::Int;
	pub use crate::nums::IntConv;
	pub use crate::nums::IntSigned;
	pub use crate::nums::IntUnsigned;
	pub use crate::nums::NumResult;
	pub use crate::nums::Overflow;
	pub use crate::nums::TryAdd;
	pub use crate::nums::TryAddAssign;
	pub use crate::nums::TryDiv;
	pub use crate::nums::TryDivAssign;
	pub use crate::nums::TryMul;
	pub use crate::nums::TryMulAssign;
	pub use crate::nums::TryRem;
	pub use crate::nums::TryRemAssign;
	pub use crate::nums::TrySub;
	pub use crate::nums::TrySubAssign;
	pub use crate::parser::*;

}
