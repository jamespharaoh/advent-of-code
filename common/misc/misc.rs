pub use arrayvec::ArrayVec;
pub use itertools;
pub use itertools::Either;
pub use itertools::EitherOrBoth;
pub use itertools::Itertools;
pub use itertools::MultiPeek;
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
pub use std::ops::BitAnd;
pub use std::ops::BitAndAssign;
pub use std::ops::BitOr;
pub use std::ops::BitOrAssign;
pub use std::ops::Deref;
pub use std::ops::Div;
pub use std::ops::Index;
pub use std::ops::IndexMut;
pub use std::ops::Mul;
pub use std::ops::Neg;
pub use std::ops::Range;
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

#[ cfg (not (fuzzing)) ]
pub use std::collections::HashSet;

#[ cfg (fuzzing) ]
pub use std::collections::BTreeSet as HashSet;

#[ cfg (not (fuzzing)) ]
pub use std::collections::HashMap;

#[ cfg (fuzzing) ]
pub use crate::test_map::HashMap;

#[ cfg (fuzzing) ]
mod test_map;

pub type GenError = Box <dyn Error>;
pub type GenResult <Ok> = Result <Ok, GenError>;

#[ must_use ]
#[ inline ]
pub fn default <T: Default> () -> T {
	Default::default ()
}

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

	pub trait IteratorResultExt <Item, Error>: Iterator <Item = Result <Item, Error>> {

		#[ inline ]
		fn collect_array_ok <const DIM: usize> (mut self) -> Result <Option <[Item; DIM]>, Error>
				where Self: Sized, Item: Copy + Default {
			let mut result = [default (); DIM];
			for idx in 0 .. DIM {
				assert! (idx < result.len ());
				result [idx] = some_or! (self.next (), return Ok (None)) ?;
			}
			if self.next ().is_some () { return Ok (None) }
			Ok (Some (result))
		}

	}

	impl <Item, Error, SomeIter: Iterator <Item = Result <Item, Error>>> IteratorResultExt <Item, Error> for SomeIter {}

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
