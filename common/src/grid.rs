use super::*;

use bitvec::BitVec;
use bitvec::BitVecEncoding;
use bitvec::BitVecIter;
use nums::Int;
use pos::PosYX;

/// Multi dimensional abstraction over a single dimensional collection
///
/// This allows a simple one-dimensional collection to be accessed in a multi-dimensional way. It
/// wraps a backing collection which implements [`GridStorage`], and takes indexes which implement
/// [`GridPos`].

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Grid <Storage, Pos, const DIMS: usize = 2> {
	storage: Storage,
	origin: [isize; DIMS],
	size: [usize; DIMS],
	phantom: PhantomData <Pos>,
}

impl <Storage, Pos, const DIMS: usize> Grid <Storage, Pos, DIMS>
	where
		Storage: GridStorage + Clone,
		Pos: GridPos <DIMS> {

	pub fn wrap (
		storage: Storage,
		origin: [isize; DIMS],
		size: [usize; DIMS],
	) -> Grid <Storage, Pos, DIMS> {
		if size.into_iter ().any (|dim| dim == 0 ) {
			panic! ("Size must be positive in all dimensions: {:?}", size);
		}
		let expected_len = size.into_iter ().product::<usize> ();
		let actual_len = storage.storage_len ();
		if expected_len != actual_len {
			panic! ("Expected {} items but was passed {}", expected_len, actual_len);
		}
		Grid { storage, origin, size, phantom: PhantomData }
	}

	pub fn len (& self) -> usize { self.size.into_iter ().product () }
	pub fn is_empty (& self) -> bool { self.size.into_iter ().any (|dim| dim == 0) }
	pub fn size (& self) -> [usize; DIMS] { self.size }

	pub fn raw_origin (& self) -> [isize; DIMS] { self.origin }
	pub fn raw_size (& self) -> [usize; DIMS] { self.size }
	pub fn origin (& self) -> Pos { Pos::from_scalar (0, self.origin, self.size).unwrap () }
	pub fn peak (& self) -> Pos { Pos::from_scalar (self.len () - 1, self.origin, self.size).unwrap () }

	pub fn get (& self, pos: Pos) -> Option <Storage::Item> {
		Pos::to_scalar (& pos, self.origin, self.size)
			.and_then (|index| self.storage.storage_get (index))
	}

	pub fn set (& mut self, pos: Pos, item: Storage::Item) {
		self.storage.storage_set (
			Pos::to_scalar (& pos, self.origin, self.size).unwrap (),
			item);
	}

	pub fn iter <'a> (& 'a self) -> GridIter <<& 'a Storage as GridStorageIntoIter>::Iter, Pos, DIMS>
			where & 'a Storage: GridStorageIntoIter {
		GridIter {
			storage: (& self.storage).storage_iter (),
			idx: 0,
			origin: self.origin,
			size: self.size,
			phantom: PhantomData,
		}
	}

	pub fn values <'a> (& 'a self) -> <& 'a Storage as GridStorageIntoIter>::Iter
			where & 'a Storage: GridStorageIntoIter {
		(& self.storage).storage_iter ()
	}

}

impl <Storage, Pos, const DIMS: usize> Grid <Storage, Pos, DIMS>
	where
		Storage: GridStorageMut + Clone,
		Pos: GridPos <DIMS> {

	pub fn get_ref (& self, pos: Pos) -> Option <& Storage::Item> {
		Pos::to_scalar (& pos, self.origin, self.size)
			.and_then (|index| self.storage.storage_ref (index))
	}

	pub fn get_mut (& mut self, pos: Pos) -> Option <& mut Storage::Item> {
		Pos::to_scalar (& pos, self.origin, self.size)
			.and_then (|index| self.storage.storage_mut (index))
	}

}

/// Iterator returned by [`Grid::iter`].
///
/// This iterator wraps the iterator from an implementation of [`GridStorage`] and maps from `Item`
/// to `([GridPos], Item)`.

pub struct GridIter <Storage, Pos, const DIMS: usize> {
	storage: Storage,
	idx: usize,
	origin: [isize; DIMS],
	size: [usize; DIMS],
	phantom: PhantomData <Pos>,
}

impl <Storage, Pos, const DIMS: usize> Iterator for GridIter <Storage, Pos, DIMS>
	where
		Storage: Iterator,
		Pos: GridPos <DIMS> {
	type Item = (Pos, Storage::Item);
	fn next (& mut self) -> Option <(Pos, Storage::Item)> {
		if let Some (item) = self.storage.next () {
			let idx = self.idx;
			self.idx += 1;
			Some ((Pos::from_scalar (idx, self.origin, self.size).unwrap (), item))
		} else { None }
	}
	fn nth (& mut self, num: usize) -> Option <(Pos, Storage::Item)> {
		if let Some (item) = self.storage.nth (num) {
			let idx = self.idx;
			self.idx += num + 1;
			Some ((Pos::from_scalar (idx, self.origin, self.size).unwrap (), item))
		} else { None }
	}
}

/// Trait for backing stores for a [`Grid`]
///
/// This provides a simple abstraction over a fixed size array of items. It is implemented for
/// [`Vec`] and [`BitVec`].

pub trait GridStorage {
	type Item;
	fn storage_get (& self, idx: usize) -> Option <Self::Item>;
	fn storage_set (& mut self, idx: usize, item: Self::Item);
	fn storage_len (& self) -> usize;
}

impl <Item> GridStorage for Vec <Item> where Item: Clone {
	type Item = Item;
	fn storage_get (& self, idx: usize) -> Option <Item> { self.get (idx).cloned () }
	fn storage_set (& mut self, idx: usize, item: Self::Item) { self [idx] = item; }
	fn storage_len (& self) -> usize { self.len () }
}

impl <Item, Encoding> GridStorage for BitVec <Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {
	type Item = Item;
	fn storage_get (& self, idx: usize) -> Option <Item> { self.get (idx) }
	fn storage_set (& mut self, idx: usize, item: Self::Item) { self.set (idx, item); }
	fn storage_len (& self) -> usize { self.len () }
}

/// Additional trait for backing stores which which can provide references to items

pub trait GridStorageMut: GridStorage {
	fn storage_ref (& self, idx: usize) -> Option <& Self::Item>;
	fn storage_mut (& mut self, idx: usize) -> Option <& mut Self::Item>;
}

impl <Item> GridStorageMut for Vec <Item> where Item: Clone {
	fn storage_ref (& self, idx: usize) -> Option <& Item> { self.get (idx) }
	fn storage_mut (& mut self, idx: usize) -> Option <& mut Item> { self.get_mut (idx) }
}

/// Extra trait for [`GridStorage`] to support iteration.
///
/// This is a separate trait to make the lifetimes work. It should be implemented on a reference to
/// the storage, rather than directly. This allows us to capture the lifetime without polluting the
/// main trait.

pub trait GridStorageIntoIter {
	type Item;
	type Iter: Iterator <Item = Self::Item>;
	fn storage_iter (& self) -> Self::Iter;
}

impl <'a, Item> GridStorageIntoIter for & 'a Vec <Item> where Item: Clone {
	type Item = Item;
	type Iter = GridStorageClone <SliceIter <'a, Item>>;
	fn storage_iter (& self) -> Self::Iter { GridStorageClone::new (self.iter ()) }
}

impl <'a, Item, Encoding> GridStorageIntoIter for & 'a BitVec <Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {
	type Item = Item;
	type Iter = BitVecIter <'a, Item, Encoding>;
	fn storage_iter (& self) -> Self::Iter { self.iter () }
}

/// Trait for values to use as indices for a [`Grid`].
///
/// For example, a two dimensional grid might be indexed with a struct containing an `x` and a `y`
/// coordinate.
///
/// This trait provides methods to translate whatever coordinate system is in use to and from a
/// single `usize` value.

pub trait GridPos <const DIMS: usize>: Sized {
	fn to_scalar (& self, origin: [isize; DIMS], size: [usize; DIMS]) -> Option <usize>;
	fn from_scalar (scalar: usize, origin: [isize; DIMS], size: [usize; DIMS]) -> Option <Self>;
}

impl GridPos <2> for (usize, usize) {
	fn to_scalar (& self, origin: [isize; 2], size: [usize; 2]) -> Option <usize> {
		if origin != [0, 0] { unimplemented! () }
		usize::checked_add (match usize::checked_mul (self.0, size [1]) {
			Some (val) => val, None => return None }, self.1)
	}
	fn from_scalar (scalar: usize, origin: [isize; 2], size: [usize; 2]) -> Option <Self> {
		if origin != [0, 0] { unimplemented! () }
		Some ((scalar / size [1], scalar % size [1]))
	}
}

impl <Val: Int> GridPos <2> for PosYX <Val> {
	fn to_scalar (& self, origin: [isize; 2], size: [usize; 2]) -> Option <usize> {
		if origin != [0, 0] { unimplemented! () }
		let y = ok_or! (self.y.to_usize (), return None);
		let x = ok_or! (self.x.to_usize (), return None);
		if y >= size [0] || x >= size [1] { return None }
		Some (y * size [1] + x)
	}
	fn from_scalar (scalar: usize, origin: [isize; 2], size: [usize; 2]) -> Option <Self> {
		if origin != [0, 0] { unimplemented! () }
		let y = ok_or! (Val::from_usize (scalar / size [1]), return None);
		let x = ok_or! (Val::from_usize (scalar % size [1]), return None);
		Some (PosYX { y, x })
	}
}

/// Wrapping iterator which clones items.
///
/// We don't use [`Cloned`](iter::Cloned) from the standard library because it doesn't handle
/// [`Iterator::skip`] the way we would like. Although it says in the documentation that there is
/// no guarantee each element will be processed, it seems like it does so. Instead, we want to
/// completely bypass any elements which aren't required.

pub struct GridStorageClone <Storage> {
	storage: Storage,
}

impl <Storage> GridStorageClone <Storage> {
	fn new (storage: Storage) -> GridStorageClone <Storage> {
		GridStorageClone { storage }
	}
}

impl <'a, Storage, Item> Iterator for GridStorageClone <Storage>
	where
		Storage: Iterator <Item = & 'a Item>,
		Item: Clone + 'a {
	type Item = Item;
	fn next (& mut self) -> Option <Item> {
		self.storage.next ().cloned ()
	}
	fn nth (& mut self, num: usize) -> Option <Item> {
		self.storage.nth (num).cloned ()
	}
}
