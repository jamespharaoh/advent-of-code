//! Generate permutations of [`usize`] from `0 .. len`.
//!
//! The main functionality is provided by the [`PermutationsHelper`] struct.
//!
//! Designed to be used with a indexed list of items of which to generate permutations, eg in a
//! [`Vec`] or similar. Provide the number of items when calling [`PermutationsHelper::new`], then
//! call [`next`](PermutationsHelper::next) in a loop, aborting if it returns `false`. After
//! calling [`next`](PermutationsHelper::next), the indexes are available by treating this struct
//! as a slice via the [`Deref`] trait.
//!
//! # Examples
//!
//! Basic usage:
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
//! This struct doesn't implement [`Iterator`] so that we avoid allocating a [`Vec`] or similar
//! for every iteration. It can easily be wrapped as one if needed:
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
//! A more typical example is generating permutations of indexes into a [`Vec`], performing a
//! calculation based on the items in that order, and finally performing some type of search or
//! aggregation over the results:
//!
//! ```
//! # use aoc_search::PermutationsHelper;
//! # use std::iter;
//! let items = vec! [ 49_u64, 93, 51, 45, 26, 73, 20, 80 ];
//! let mut perms_helper = PermutationsHelper::new (items.len ());
//! let calc_iter = iter::from_fn (move ||
//!   perms_helper.next ().then (|| perms_helper.iter ()
//!     .map (|& idx| items [idx])
//!     .fold (0, |sum, item| sum * 100 + item)));
//! assert_eq! (9380735149452620, calc_iter.clone ().max ().unwrap ());
//! assert_eq! (2026454951738093, calc_iter.clone ().min ().unwrap ());
//! ```
//!
//! # Algorithm
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

/// The `PermutationsHelper` type. See the [module level documentation](self) for more
/// information.
///
#[ derive (Clone, Debug) ]
pub struct PermutationsHelper <const CIR: bool = false, const MIR: bool = false> {
	data: Vec <usize>,
	first: bool,
}

impl PermutationsHelper <false, false> {

	/// Create a new [`PermutationsHelper`].
	///
	/// # Examples
	///
	/// ```
	/// # use aoc_search::PermutationsHelper;
	/// # use std::iter;
	/// let mut perms_helper = PermutationsHelper::new (3);
	/// let mut perms_iter = iter::from_fn (move ||
	///   perms_helper.next ().then (|| perms_helper.to_vec ()));
	/// assert_eq! (Some (vec! [0, 1, 2]), perms_iter.next ());
	/// assert_eq! (Some (vec! [0, 2, 1]), perms_iter.next ());
	/// assert_eq! (Some (vec! [1, 0, 2]), perms_iter.next ());
	/// assert_eq! (Some (vec! [1, 2, 0]), perms_iter.next ());
	/// assert_eq! (Some (vec! [2, 0, 1]), perms_iter.next ());
	/// assert_eq! (Some (vec! [2, 1, 0]), perms_iter.next ());
	/// assert_eq! (None, perms_iter.next ());
	/// ```
	///
	#[ inline ]
	#[ must_use ]
	pub fn new (len: usize) -> Self {
		Self {
			data: (0 .. len).collect (),
			first: true,
		}
	}

}

impl PermutationsHelper <true, false> {

	/// Create a new [`PermutationsHelper`] in "circular" mode.
	///
	/// This will always return index `0` in the first position, and is intended for applications
	/// where rotating the permutation by any amount doesn't change its meaning.
	///
	/// This reduces the number of iterations by a factor of the number of items.
	///
	/// # Examples
	///
	/// ```
	/// # use aoc_search::PermutationsHelper;
	/// # use std::iter;
	/// let mut perms_helper = PermutationsHelper::new_circular (3);
	/// let mut perms_iter = iter::from_fn (move ||
	///   perms_helper.next ().then (|| perms_helper.to_vec ()));
	/// assert_eq! (Some (vec! [0, 1, 2]), perms_iter.next ());
	/// assert_eq! (Some (vec! [0, 2, 1]), perms_iter.next ());
	/// assert_eq! (None, perms_iter.next ());
	/// ```
	///
	#[ inline ]
	#[ must_use ]
	pub fn new_circular (len: usize) -> Self {
		Self {
			data: (0 .. len).collect (),
			first: true,
		}
	}

}

impl PermutationsHelper <false, true> {

	/// Create a new [`PermutationsHelper`] in "mirror" mode.
	///
	/// This will always permutations with the first position lower than the last, and is intended
	/// for applications where reversing the permutation doesn't change its meaning. This reduces
	/// the number of iterations by a factor of two.
	///
	/// Note that there is no clever optimisation here, we simply loop over the skipped iterations
	/// after calculating them. This may be useful, however, if an expensive calcualtion is
	/// performed for each one.
	///
	/// # Examples
	///
	/// ```
	/// # use aoc_search::PermutationsHelper;
	/// # use std::iter;
	/// let mut perms_helper = PermutationsHelper::new_mirror (3);
	/// let mut perms_iter = iter::from_fn (move ||
	///   perms_helper.next ().then (|| perms_helper.to_vec ()));
	/// assert_eq! (Some (vec! [0, 1, 2]), perms_iter.next ());
	/// assert_eq! (Some (vec! [0, 2, 1]), perms_iter.next ());
	/// assert_eq! (Some (vec! [1, 0, 2]), perms_iter.next ());
	/// assert_eq! (None, perms_iter.next ());
	/// ```
	///
	#[ inline ]
	#[ must_use ]
	pub fn new_mirror (len: usize) -> Self {
		Self {
			data: (0 .. len).collect (),
			first: true,
		}
	}

}

impl <const CIR: bool, const MIR: bool> PermutationsHelper <CIR, MIR> {

	/// Find the next permutation in the sequence.
	///
	/// This returns `true` if another permutation was found, and `false` otherwise. This is
	/// typically called in a loop which accesses the permutation as a [`prim@slice`] via the
	/// [`Deref`] trait
	///
	/// # Examples
	///
	/// ```
	/// # use aoc_search::PermutationsHelper;
	/// # let mut perms_helper = PermutationsHelper::new (4);
	/// while perms_helper.next () {
	///   println! ("{:?}", & * perms_helper);
	/// }
	/// ```
	///
	#[ allow (clippy::should_implement_trait) ]
	#[ inline ]
	pub fn next (& mut self) -> bool {

		// special case for first iteration

		if self.first {
			self.first = false;
			return ! self.data.is_empty ();
		}

		loop {

			// find first index to update

			let (idx_0, val_0) = some_or! (
				self.data.iter ().rev ()
					.array_windows ()
					.enumerate ()
					.find (|& (_, [& val_1, & val_0])| val_0 < val_1)
					.map (|(off_0, [_, & val_0])| (self.data.len () - 2 - off_0, val_0)),
				return false);

			if CIR && idx_0 == 0 { return false }

			// find second index to update

			let (idx_1, val_1) = some_or! (
				self.data [idx_0 + 1 .. ].iter ().rev ()
					.enumerate ()
					.find (|& (_, & val_1)| val_0 < val_1)
					.map (|(off_1, & val_1)| (self.data.len () - 1 - off_1, val_1)),
				return false);

			// swap the values at the two indexes

			self.data [idx_0] = val_1;
			self.data [idx_1] = val_0;

			// reverse everything after the first index

			self.data [idx_0 + 1 .. ].reverse ();

			// skip mirror duplicates if that option is enabled

			if ! MIR || self.data [0] < self.data [self.data.len () - 1] { break }

		}

		true

	}

}

impl <const CIR: bool, const MIR: bool> Deref for PermutationsHelper <CIR, MIR> {

	type Target = [usize];

	#[ inline ]
	fn deref (& self) -> & [usize] {
		& self.data
	}

}
