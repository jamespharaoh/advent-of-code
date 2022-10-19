use std::fmt::Debug;
use std::marker::PhantomData;
use std::slice::Iter as SliceIter;

use aoc_bitvec as bitvec;
use aoc_checked::checked as chk;
use aoc_misc::prelude::*;
use aoc_nums as nums;
use aoc_parser::*;

use bitvec::BitVec;
use bitvec::BitVecEncoding;
use bitvec::BitVecIter;
use nums::Int;
use nums::IntConv;
use nums::IntSigned;
use nums::NumResult;
use nums::Overflow;
use nums::TryAdd;
use nums::TryAddAssign;
use nums::TryMul;
use nums::TrySub;
use nums::TrySubAssign;

mod buf;
mod cursor;
mod display;
mod extend;
mod iter;
mod parse;
mod pos;
mod storage;
mod transform;
mod view;

pub use buf::*;
pub use cursor::*;
pub use display::*;
pub use extend::*;
pub use iter::*;
pub use parse::*;
pub use pos::*;
pub use storage::*;
pub use transform::*;
pub use view::*;

pub mod prelude {
	pub use crate::GridBuf;
	pub use crate::GridCursor;
	pub use crate::GridCursorCompact;
	pub use crate::GridOffset;
	pub use crate::GridPos;
	pub use crate::GridView;
	pub use crate::GridViewExtend;
	pub use crate::GridViewIter;
	pub use crate::GridViewPrint;
}
