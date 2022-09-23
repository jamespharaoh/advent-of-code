use std::fmt::Debug;
use std::marker::PhantomData;
use std::slice::Iter as SliceIter;

use aoc_bitvec as bitvec;
use aoc_misc::*;
use aoc_nums as nums;
use aoc_parser::*;

use bitvec::BitVec;
use bitvec::BitVecEncoding;
use bitvec::BitVecIter;
use nums::Int;
use nums::IntConv;
use nums::Overflow;
use nums::TryAdd;

mod cursor;
mod display;
mod iter;
mod parse;
mod pos;
mod storage;

pub use cursor::GridCursorIter;
pub use cursor::GridCursor;
pub use cursor::GridCursorMut;
pub use cursor::GridOffset;
pub use pos::GridPos;
pub use pos::GridPosDisplay;
pub use pos::GridPosDisplayOrder;
pub use iter::GridIter;
pub use iter::GridKeysIter;
pub use iter::GridStorageClone;
pub use storage::GridStorage;
pub use storage::GridStorageIntoIter;
pub use storage::GridStorageMut;

/// Multi dimensional abstraction over a single dimensional collection
///
/// This allows a simple one-dimensional collection to be accessed in a multi-dimensional way. It
/// wraps a backing collection which implements [`GridStorage`], and takes indexes which implement
/// [`GridPos`].

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Grid <Storage, Pos, const DIMS: usize = 2> {
	storage: Storage,
	origin: [isize; DIMS],
	size: [usize; DIMS],
	phantom: PhantomData <Pos>,
}

impl <Item, Pos, const DIMS: usize> Grid <Vec <Item>, Pos, DIMS>
	where
		Item: Clone + Default,
		Pos: GridPos <DIMS> {

	#[ inline ]
	#[ must_use ]
	pub fn new_vec (
		origin: [isize; DIMS],
		size: [usize; DIMS],
	) -> Self {
		Self::wrap (
			std::iter::repeat (default ())
				.take (size.iter ().copied ().product ())
				.collect::<Vec <_>> (),
			origin,
			size)
	}

	#[ inline ]
	pub fn reset (& mut self) {
		for item in self.storage.iter_mut () {
			* item = default ();
		}
	}

	#[ inline ]
	#[ must_use ]
	pub fn resize (& self, origin: [isize; DIMS], size: [usize; DIMS]) -> Self {
		Self::wrap (
			GridKeysIter::new (origin, size)
				.map (|pos| self.get (pos).unwrap_or_default ())
				.collect (),
			origin,
			size)
	}

}

impl <Storage, Pos, const DIMS: usize> Grid <Storage, Pos, DIMS>
	where
		Storage: GridStorage + Clone,
		Pos: GridPos <DIMS> {

	#[ inline ]
	#[ must_use ]
	pub fn validate_dims (origin: [isize; DIMS], size: [usize; DIMS]) -> bool {
		Pos::from_scalar (0, origin, size).is_some ()
			&& Pos::from_scalar (size.iter ().copied ().product::<usize> () - 1, origin, size).is_some ()
	}

	#[ inline ]
	pub fn wrap (
		storage: Storage,
		origin: [isize; DIMS],
		size: [usize; DIMS],
	) -> Self {
		assert! (! size.into_iter ().any (|dim| dim == 0),
			"Size must be positive in all dimensions: {:?}", size);
		let expected_len = size.into_iter ().product::<usize> ();
		let actual_len = storage.storage_len ();
		assert! (expected_len == actual_len,
			"Expected {} items but was passed {}", expected_len, actual_len);
		Self { storage, origin, size, phantom: PhantomData }
	}

	#[ inline ]
	pub fn len (& self) -> usize {
		self.size.into_iter ().product ()
	}

	#[ inline ]
	pub fn is_empty (& self) -> bool {
		self.size.into_iter ().any (|dim| dim == 0)
	}

	#[ inline ]
	pub const fn native_origin (& self) -> [isize; DIMS] {
		self.origin
	}

	#[ inline ]
	pub const fn native_size (& self) -> [usize; DIMS] {
		self.size
	}

	#[ inline ]
	pub fn size (& self) -> Pos {
		Pos::size_from_native (self.size).unwrap ()
	}

	#[ inline ]
	pub fn first_key (& self) -> Pos {
		Pos::from_scalar (0, self.origin, self.size).unwrap ()
	}

	#[ inline ]
	pub fn last_key (& self) -> Pos {
		Pos::from_scalar (self.len () - 1, self.origin, self.size).unwrap ()
	}

	#[ inline ]
	pub fn get (& self, pos: Pos) -> Option <Storage::Item> {
		Pos::to_scalar (pos, self.origin, self.size)
			.and_then (|index| self.storage.storage_get (index))
	}

	#[ inline ]
	pub fn get_native (& self, pos: [usize; DIMS]) -> Option <Storage::Item> {
		let mut idx = 0;
		for (val, size) in pos.iter ().copied ().zip (self.size.iter ().copied ()) {
			if size <= val { return None }
			idx = idx * size + val;
		}
		self.storage.storage_get (idx)
	}

	#[ inline ]
	pub fn set (& mut self, pos: Pos, item: Storage::Item) {
		let idx = some_or! (
			Pos::to_scalar (pos, self.origin, self.size),
			panic! ("Unable to map GridPos to scalar: {:?}", pos));
		self.storage.storage_set (idx, item);
	}

	#[ inline ]
	pub fn try_set (& mut self, pos: Pos, item: Storage::Item) -> Option <()> {
		let idx = Pos::to_scalar (pos, self.origin, self.size) ?;
		self.storage.storage_set (idx, item);
		Some (())
	}

	#[ inline ]
	pub fn iter <'sto> (& 'sto self) -> GridIter <Storage, Pos, DIMS>
			where & 'sto Storage: GridStorageIntoIter {
		Iterator::zip (self.keys (), self.values ())
	}

	#[ inline ]
	pub fn values <'sto> (& 'sto self) -> <& 'sto Storage as GridStorageIntoIter>::Iter
			where & 'sto Storage: GridStorageIntoIter {
		(& self.storage).storage_iter ()
	}

	#[ inline ]
	pub const fn keys (& self) -> GridKeysIter <Pos, DIMS> {
		GridKeysIter::new (self.origin, self.size)
	}

	#[ inline ]
	#[ must_use ]
	pub fn map <'grd, MapFn, Output> (& 'grd self, map_fn: MapFn) -> Self
		where
			Storage: FromIterator <Output>,
			& 'grd Storage: GridStorageIntoIter,
			MapFn: FnMut (GridCursor <'grd, Storage, Pos, DIMS>) -> Output {
		Self {
			storage: self.cursors ().map (map_fn).collect (),
			origin: self.origin,
			size: self.size,
			phantom: PhantomData,
		}
	}

	#[ inline ]
	pub const fn cursors (& self) -> GridCursorIter <Storage, Pos, DIMS> {
		GridCursorIter {
			grid: self,
			pos: [0; DIMS],
			idx: 0,
			done: false,
			phantom: PhantomData,
		}
	}

	#[ inline ]
	pub fn offset (& self, pos: Pos) -> GridOffset <DIMS> {
		GridOffset::new (self.size, pos)
	}

}

impl <Storage, Pos, const DIMS: usize> Grid <Storage, Pos, DIMS>
	where
		Storage: GridStorageMut + Clone,
		Pos: GridPos <DIMS> {

	#[ inline ]
	pub fn get_ref (& self, pos: Pos) -> Option <& Storage::Item> {
		Pos::to_scalar (pos, self.origin, self.size)
			.and_then (|index| self.storage.storage_ref (index))
	}

	#[ inline ]
	pub fn get_mut (& mut self, pos: Pos) -> Option <& mut Storage::Item> {
		Pos::to_scalar (pos, self.origin, self.size)
			.and_then (|index| self.storage.storage_mut (index))
	}

	#[ inline ]
	pub fn cursor_mut (& mut self, pos: Pos) -> Option <GridCursorMut <Storage, Pos, DIMS>> {
		let idx = pos.to_scalar (self.origin, self.size) ?;
		let pos = pos.to_native (self.origin) ?;
		Some (GridCursorMut::new (self, pos, idx))
	}

}
