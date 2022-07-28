#![ doc (html_playground_url = "https://play.rust-lang.org/") ]

use clap::ArgMatches;
use clap::Command;

#[ doc (no_inline) ]
pub use prelude::*;

pub mod bithash;
pub mod bitvec;
pub mod grid;
pub mod list;
pub mod nums;
pub mod parser;
pub mod pos;
pub mod puzzle;
pub mod search;

pub type GenError = Box <dyn Error>;
pub type GenResult <Ok> = Result <Ok, GenError>;

#[ must_use ]
#[ inline ]
pub fn default <T: Default> () -> T {
	Default::default ()
}

mod assertions {

	#[ macro_export ]
	macro_rules! ok_or {
		( $val:expr, $if_err:expr ) => {
			match ($val) { Ok (val) => val, Err (_) => $if_err }
		};
	}

	#[ macro_export ]
	macro_rules! some_or {
		( $val:expr, $if_err:expr ) => {
			match ($val) { Some (val) => val, None => $if_err }
		};
	}

	#[ macro_export ]
	macro_rules! assert_is_ok {
		( $actual:expr ) => {
			assert! ($actual.is_ok ());
		};
	}

	#[ macro_export ]
	macro_rules! assert_eq_ok {
		( $expect:expr , $actual:expr ) => {
			let actual = $actual;
			assert! (actual.is_ok ());
			assert_eq! ($expect, actual.unwrap ());
		};
	}

	#[ macro_export ]
	macro_rules! assert_err {
		( $expect:expr , $actual:expr ) => {
			assert_eq! ($expect, $actual.unwrap_err ().to_string ());
		};
	}

}

#[ macro_export ]
macro_rules! array_vec {
	( ) => { ArrayVec::new () };
	( $($vals:expr),+ ) => {
		{
			let mut result = ArrayVec::new ();
			array_vec! (@push result, $($vals,)*);
			result
		}
	};
	( @push $result:ident $(,)? ) => {};
	( @push $result:ident , $val:expr $(, $rest:expr)* ) => {
		$result.push ($val);
		array_vec! (@push $result, $($rest),*);
	};
	( @push $result:ident , $val:expr $(, $rest:expr)* , ) => {
		$result.push ($val);
		array_vec! (@push $result, $($rest),*);
	};
}

mod iter_ext {

	use super::*;
	use iter::Copied;

	pub trait IntoIteratorExt: IntoIterator + Sized {
		fn iter_vals <'a, Item> (self) -> Copied <Self::IntoIter>
			where
				Item: 'a + Copy,
				Self: IntoIterator <Item = & 'a Item> {
			self.into_iter ().copied ()
		}
	}

	impl <'a, IntoIter> IntoIteratorExt for & 'a IntoIter where & 'a IntoIter: IntoIterator {}

	pub trait IteratorExt: Iterator {
		fn collect_array <const DIM: usize> (mut self) -> Option <[Self::Item; DIM]>
				where Self: Sized, Self::Item: Copy + Default {
			let mut result = [default (); DIM];
			for idx in 0 .. DIM {
				assert! (idx < result.len ());
				result [idx] = some_or! (self.next (), return None);
			}
			if self.next ().is_some () { return None }
			Some (result)
		}
	}

	impl <SomeIter: Iterator> IteratorExt for SomeIter {}

}

mod prelude {
	pub use arrayvec::ArrayVec;
	pub use clap;
	pub use itertools;
	pub use itertools::Either;
	pub use itertools::EitherOrBoth;
	pub use itertools::Itertools;
	pub use itertools::izip;
	pub use std::borrow::Borrow;
	pub use std::borrow::Cow;
	pub use std::cell::Cell;
	pub use std::cell::RefCell;
	pub use std::cmp;
	pub use std::cmp::Ordering;
	pub use std::collections::BinaryHeap;
	pub use std::collections::HashMap;
	pub use std::collections::HashSet;
	pub use std::collections::VecDeque;
	pub use std::collections::hash_map::DefaultHasher;
	pub use std::collections::hash_map::RandomState as RandomHasher;
	pub use std::convert::Infallible;
	pub use std::error::Error;
	pub use std::ffi::OsString;
	pub use std::fmt;
	pub use std::fmt::Debug;
	pub use std::fmt::Display;
	pub use std::fs;
	pub use std::hash;
	pub use std::hash::BuildHasher;
	pub use std::hash::BuildHasherDefault;
	pub use std::hash::Hash;
	pub use std::hash::Hasher;
	pub use std::iter;
	pub use std::iter::Peekable;
	pub use std::marker::PhantomData;
	pub use std::mem;
	pub use std::ops;
	pub use std::ops::Add;
	pub use std::ops::Deref;
	pub use std::ops::Div;
	pub use std::ops::Index;
	pub use std::ops::IndexMut;
	pub use std::ops::Mul;
	pub use std::ops::Neg;
	pub use std::ops::Range;
	pub use std::ops::RangeInclusive;
	pub use std::ops::Rem;
	pub use std::ops::Sub;
	pub use std::path::Path;
	pub use std::rc::Rc;
	pub use std::rc::Weak as RcWeak;
	pub use std::slice;
	pub use std::slice::Iter as SliceIter;
	pub use std::str::Chars;
	pub use std::str::FromStr;
	pub use std::sync::Arc;
	pub use std::sync::Condvar;
	pub use std::sync::Mutex;
	pub use std::thread;
	pub use std::time;
	pub use crate::iter_ext::IntoIteratorExt as _;
	pub use crate::iter_ext::IteratorExt as _;
}
