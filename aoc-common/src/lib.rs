use clap::ArgMatches;
use clap::Command;

pub use arrayvec::ArrayVec;
pub use clap;
pub use itertools;
pub use itertools::Either;
pub use itertools::Itertools;
pub use num;
pub use std::borrow::Borrow;
pub use std::borrow::Cow;
pub use std::cell::RefCell;
pub use std::cmp;
pub use std::collections::BinaryHeap;
pub use std::collections::HashMap;
pub use std::collections::HashSet;
pub use std::collections::VecDeque;
pub use std::error::Error;
pub use std::ffi::OsString;
pub use std::fmt;
pub use std::fs;
pub use std::hash;
pub use std::iter;
pub use std::iter::Peekable;
pub use std::marker::PhantomData;
pub use std::mem;
pub use std::ops;
pub use std::ops::Range;
pub use std::ops::RangeInclusive;
pub use std::path::Path;
pub use std::rc::Rc;
pub use std::rc::Weak as RcWeak;
pub use std::slice;
pub use std::slice::Iter as SliceIter;
pub use std::str::Chars;
pub use std::str::FromStr;
pub use std::time;

pub mod grid;
pub mod parser;
pub mod pos;
pub mod puzzle;
pub mod search;

mod puzzle_macro;

pub type GenError = Box <dyn Error>;
pub type GenResult <Ok> = Result <Ok, GenError>;

#[ must_use ]
#[ inline ]
pub fn default <T: Default> () -> T {
	Default::default ()
}
