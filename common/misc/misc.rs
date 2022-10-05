pub use arrayvec::ArrayVec;
pub use itertools;
pub use itertools::Either;
pub use itertools::EitherOrBoth;
pub use itertools::Itertools;
pub use itertools::MultiPeek;
pub use itertools::izip;
pub use std::array;
pub use std::borrow::Borrow;
pub use std::borrow::BorrowMut;
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
pub use std::collections::btree_map::IterMut as BTreeIterMut;
pub use std::collections::btree_map::Keys as BTreeKeys;
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
pub use std::iter::FusedIterator;
pub use std::iter::Peekable;
pub use std::io;
pub use std::marker::PhantomData;
pub use std::mem;
pub use std::num::ParseIntError;
pub use std::ops;
pub use std::ops::Add;
pub use std::ops::AddAssign;
pub use std::ops::BitAnd;
pub use std::ops::BitAndAssign;
pub use std::ops::BitOr;
pub use std::ops::BitOrAssign;
pub use std::ops::Bound;
pub use std::ops::Bound::Included;
pub use std::ops::Bound::Excluded;
pub use std::ops::Bound::Unbounded;
pub use std::ops::Deref;
pub use std::ops::DerefMut;
pub use std::ops::Div;
pub use std::ops::Index;
pub use std::ops::IndexMut;
pub use std::ops::Mul;
pub use std::ops::Neg;
pub use std::ops::Range;
pub use std::ops::RangeBounds;
pub use std::ops::RangeInclusive;
pub use std::ops::Rem;
pub use std::ops::Shl;
pub use std::ops::ShlAssign;
pub use std::ops::Shr;
pub use std::ops::ShrAssign;
pub use std::ops::Sub;
pub use std::path::Path;
pub use std::rc::Rc;
pub use std::rc::Weak as RcWeak;
pub use std::slice;
pub use std::slice::Iter as SliceIter;
pub use std::slice::IterMut as SliceIterMut;
pub use std::str;
pub use std::str::Chars;
pub use std::str::FromStr;
pub use std::sync::Arc;
pub use std::sync::Condvar;
pub use std::sync::Mutex;
pub use std::sync::atomic::AtomicUsize;
pub use std::sync::atomic::Ordering as AtomicOrdering;
pub use std::thread;
pub use std::thread::JoinHandle;
pub use std::time;
pub use std::vec::IntoIter as VecIntoIter;

#[ cfg (not (fuzzing)) ]
//pub use std_hash::*;
pub use fast_hash::*;

#[ cfg (fuzzing) ]
pub use test_hash::*;

mod std_hash {
	pub use std::collections::HashMap;
	pub use std::collections::HashSet;
}

mod fast_hash {
	pub use ahash::AHashMap as HashMap;
	pub use ahash::AHashSet as HashSet;
}

#[ cfg (fuzzing) ]
mod test_hash {
	pub use std::collections::BTreeSet as HashSet;
	pub use crate::test_map::HashMap;
}

#[ cfg (fuzzing) ]
mod test_map;

pub use crate::iter_ext::*;

pub type GenError = Box <dyn Error>;
pub type GenResult <Ok> = Result <Ok, GenError>;

#[ inline ]
#[ must_use ]
pub fn default <T: Default> () -> T {
	Default::default ()
}

pub trait ResultEither <Val> {
	fn either (self) -> Val;
}

impl <Val> ResultEither <Val> for Result <Val, Val> {
	#[ inline ]
	#[ must_use ]
	fn either (self) -> Val {
		match self { Ok (val) => val, Err (val) => val }
	}
}

#[ allow (clippy::missing_const_for_fn) ]
#[ inline ]
#[ must_use ]
pub fn ok_or_err <Val> (result: Result <Val, Val>) -> Val {
	match result {
		Ok (val) => val,
		Err (val) => val,
	}
}

#[ macro_export ]
macro_rules! ok_or {
	( $val:expr, $if_err:expr $(,)? ) => {
		match ($val) { Ok (val) => val, Err (_) => $if_err }
	};
}

#[ macro_export ]
macro_rules! some_or {
	( $val:expr, $if_err:expr $(,)? ) => {
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
		assert! (actual.is_ok (), "Expected Ok but got {:?}", actual);
		assert_eq! ($expect, actual.unwrap ());
	};
	( $expect: expr , $actual:expr , $($rest:tt)* ) => {
		let actual = $actual;
		assert! (actual.is_ok (), $($rest)*);
		assert_eq! ($expect, actual.unwrap ());
	};
}

#[ macro_export ]
macro_rules! assert_err {
	( $expect:expr , $actual:expr ) => {
		assert_eq! ($expect, $actual.unwrap_err ().to_string ());
	};
}

mod iter_ext {

	use super::*;

	pub trait IteratorExt: Iterator {

		#[ inline ]
		fn array <const DIM: usize> (mut self) -> [Self::Item; DIM]
				where Self: Sized, Self::Item: Copy + Default {
			let mut result = [default (); DIM];
			for idx in 0 .. DIM {
				result [idx] = self.next ().unwrap ();
			}
			assert! (self.next ().is_none ());
			result
		}

		#[ inline ]
		fn try_array <Item, Error, const DIM: usize> (mut self) -> Result <[Item; DIM], Error>
				where Self: Sized + Iterator <Item = Result <Item, Error>>, Item: Copy + Default {
			let mut result = [default (); DIM];
			for idx in 0 .. DIM {
				result [idx] = self.next ().unwrap () ?;
			}
			assert! (self.next ().is_none ());
			Ok (result)
		}

	}

	impl <SomeIter: Iterator> IteratorExt for SomeIter {}

}

#[ macro_export ]
macro_rules! array_vec {
	() => { ArrayVec::new () };
	( $($val:expr),* $(,)? ) => {
		{
			let mut result = ArrayVec::new ();
			$( result.push ($val); )*
			result
		}
	};
}

#[ macro_export ]
macro_rules! wrapper_deref_mut {
	(
		$(#[$struct_meta:meta])*
		$struct_vis:vis struct $struct_name:ident $(<$($struct_param:tt),*>)? {
			$field_vis:vis $field_name:ident: $field_type:ty,
		}
	) => {

		$(#[$struct_meta])*
		$struct_vis struct $struct_name $(<$($struct_param),*>)? {
			$field_vis $field_name: $field_type,
		}

		impl $(<$($struct_param),*>)? ::std::ops::Deref for $struct_name $(<$($struct_param),*>)? {
			type Target = $field_type;
			fn deref (& self) -> & $field_type {
				& self.$field_name
			}
		}

		impl $(<$($struct_param),*>)? ::std::ops::DerefMut for $struct_name $(<$($struct_param),*>)? {
			fn deref_mut (& mut self) -> & mut $field_type {
				& mut self.$field_name
			}
		}

	};
}
