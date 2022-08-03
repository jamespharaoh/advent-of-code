//! Common functionality used in all puzzle solutions

use clap::ArgMatches;
use clap::Command;

#[ doc (no_inline) ]
pub use prelude::*;

pub mod bithash;
pub mod bitvec;
pub mod grid;
pub mod list;
pub mod md5;
pub mod nums;
pub mod ocr;
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

		#[ inline ]
		fn iter_vals <'dat, Item> (self) -> Copied <Self::IntoIter>
			where
				Item: 'dat + Copy,
				Self: IntoIterator <Item = & 'dat Item> {
			self.into_iter ().copied ()
		}

	}

	impl <'dat, IntoIter> IntoIteratorExt for & 'dat IntoIter
		where & 'dat IntoIter: IntoIterator {}

	pub trait IteratorExt: Iterator {

		#[ inline ]
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
	pub use std::collections::BTreeMap;
	pub use std::collections::BTreeSet;
	pub use std::collections::VecDeque;
	pub use std::collections::btree_map::Entry as BTreeEntry;
	pub use std::collections::btree_map::Iter as BTreeIter;
	pub use std::collections::btree_map::Values as BTreeValues;
	pub use std::collections::hash_map::DefaultHasher;
	pub use std::collections::hash_map::Entry as HashMapEntry;
	pub use std::collections::hash_map::RandomState as RandomHasher;
	pub use std::convert::Infallible;
	pub use std::error::Error;
	pub use std::ffi::OsString;
	pub use std::fmt;
	pub use std::fmt::Debug;
	pub use std::fmt::Display;
	pub use std::fmt::Write as _;
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
	pub use crate::nums::Int;
	pub use crate::nums::IntConv as _;

	#[ cfg (not (fuzzing)) ]
	pub use std::collections::HashSet;
	#[ cfg (not (fuzzing)) ]
	pub use std::collections::HashMap;

	#[ cfg (fuzzing) ]
	pub use crate::test_map::HashMap;

	#[ cfg (fuzzing) ]
	pub use std::collections::BTreeSet as HashSet;

}

#[ cfg (fuzzing) ]
mod test_map {

	use super::*;

	pub struct HashMap <Key, Val, Hshr = RandomHasher> {
		map: BTreeMap <Key, Val>,
		phantom: PhantomData <Hshr>,
	}

	impl <Key, Val, Hshr> HashMap <Key, Val, Hshr>
		where Key: Ord {

		pub fn new () -> Self {
			Self {
				map: BTreeMap::new (),
				phantom: PhantomData,
			}
		}

		pub fn get <Qry> (& self, key: & Qry) -> Option <& Val>
			where
				Key: Borrow <Qry>,
				Qry: Eq + Hash + Ord + ?Sized {
			self.map.get (key)
		}

		pub fn get_mut <Qry> (& mut self, key: & Qry) -> Option <& mut Val>
			where
				Key: Borrow <Qry>,
				Qry: Eq + Hash + Ord + ?Sized {
			self.map.get_mut (key)
		}

		pub fn entry (& mut self, key: Key) -> BTreeEntry <'_, Key, Val> {
			self.map.entry (key)
		}

		pub fn insert (& mut self, key: Key, val: Val) -> Option <Val> {
			self.map.insert (key, val)
		}

		pub fn iter (& self) -> BTreeIter <'_, Key, Val> {
			self.map.iter ()
		}

		pub fn len (& self) -> usize {
			self.map.len ()
		}

		pub fn values (& self) -> BTreeValues <Key, Val> {
			self.map.values ()
		}

	}

	impl <Key, Val> Clone for HashMap <Key, Val> where Key: Clone, Val: Clone {
		fn clone (& self) -> Self {
			Self {
				map: self.map.clone (),
				phantom: PhantomData,
			}
		}
	}

	impl <Key, Val> Debug for HashMap <Key, Val>
			where Key: Debug, Val: Debug {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			self.map.fmt (formatter)
		}
	}

	impl <Key, Val> Default for HashMap <Key, Val> {
		fn default () -> Self {
			Self {
				map: BTreeMap::default (),
				phantom: PhantomData,
			}
		}
	}

	impl <Key, Val> FromIterator <(Key, Val)> for HashMap <Key, Val>
			where Key: Ord {
		fn from_iter <Iter> (iter: Iter) -> Self
				where Iter: IntoIterator <Item = (Key, Val)> {
			Self {
				map: BTreeMap::from_iter (iter),
				phantom: PhantomData,
			}
		}
	}

	impl <Key, Val, Qry> Index <& '_ Qry> for HashMap <Key, Val>
		where
			Key: Ord + Borrow <Qry>,
			Qry: Ord + ?Sized {
		type Output = Val;
		fn index (& self, query: & Qry) -> & Val {
			self.map.get (query).unwrap ()
		}
	}

}
