//! Generate permutations of `usize` from `0 .. len`
//!
//! Designed to be used with a indexed list of items to generate permutations of, eg in a [`Vec`].
//! Provide the number of items when calling `new`, then call `next` in a loop, aborting if it
//! returns `false`. After calling `next`, the indexes are available by treating this struct as a
//! slice via the [`Deref`] trait.
//!
//! # Sample usage
//!
//! ```
//! # use aoc_search::PermutationsHelper;
//! let mut perms_helper = PermutationsHelper::new (2);
//! assert_eq! (true, perms_helper.next ());
//! assert_eq! (& [0, 1], & * perms_helper);
//! assert_eq! (true, perms_helper.next ());
//! assert_eq! (& [1, 0], & * perms_helper);
//! assert_eq! (false, perms_helper.next ());
//! ```
//!
//! This is not implemented as an iterator so that we avoid allocating a `Vec` or similar for
//! every iteration. It can easily be wrapped as one if needed:
//!
//! ```
//! # use aoc_search::PermutationsHelper;
//! # use std::iter;
//! let mut perms_helper = PermutationsHelper::new (2);
//! let mut perms_iter = iter::from_fn (move ||
//!   perms_helper.next ().then (|| perms_helper.to_vec ()));
//! assert_eq! (Some (vec! [0, 1]), perms_iter.next ());
//! assert_eq! (Some (vec! [1, 0]), perms_iter.next ());
//! assert_eq! (None, perms_iter.next ());
//! ```
//!
//! ## Algorithm
//!
//! This implements a "classic" algorithm described on
//! [Wikipedia](https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order),
//! which produces results in lexicographical order. The alogirthm starts with the items in order,
//! then applies the following steps to generate subsequent iterations:
//!
//! * Find the highest index `idx_0` where `items [idx_0] < items [idx_0 + 1]`
//! * Find the highest index `idx_1` where `idx_0 < idx_1 && items [idx_0] < items [idx_1]`
//! * Swap `items [idx_0]` and `items [idx_1]`
//! * Reverse the order of `items [idx_0 + 1 .. ]`

use super::*;

#[ derive (Clone, Debug) ]
pub struct PermutationsHelper {
	data: Vec <usize>,
	first: bool,
}

impl PermutationsHelper {

	#[ inline ]
	#[ must_use ]
	pub fn new (len: usize) -> Self {
		Self {
			data: (0 .. len).collect (),
			first: true,
		}
	}

	#[ allow (clippy::should_implement_trait) ]
	#[ inline ]
	pub fn next (& mut self) -> bool {

		// special case for first iteration

		if self.first {
			self.first = false;
			return ! self.data.is_empty ();
		}

		// find first index to update

		let (idx_0, val_0) = some_or! (
			self.data.iter ().rev ()
				.tuple_windows ()
				.enumerate ()
				.find (|& (_, (& val_1, & val_0))| val_0 < val_1)
				.map (|(off_0, (_, & val_0))| (self.data.len () - 2 - off_0, val_0)),
			return false);

		// find second index to update

		let (idx_1, val_1) =
			self.data [idx_0 + 1 .. ].iter ().rev ()
				.enumerate ()
				.find (|& (_, & val_1)| val_0 < val_1)
				.map (|(off_1, & val_1)| (self.data.len () - 1 - off_1, val_1))
				.unwrap ();

		// swap the values at the two indexes

		self.data [idx_0] = val_1;
		self.data [idx_1] = val_0;

		// reverse everything after the first index

		self.data [idx_0 + 1 .. ].reverse ();

		true

	}

}

impl Deref for PermutationsHelper {

	type Target = [usize];

	#[ inline ]
	fn deref (& self) -> & [usize] {
		& self.data
	}

}
