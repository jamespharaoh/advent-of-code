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
pub struct Grid <Inner, Pos, const DIMS: usize = 2> {
	inner: Inner,
	origin: [isize; DIMS],
	size: [usize; DIMS],
	phantom: PhantomData <Pos>,
}

impl <Inner, Pos, const DIMS: usize> Grid <Inner, Pos, DIMS>
	where
		Inner: GridStorage + Clone,
		Pos: GridPos <DIMS> {

	pub fn wrap (
		inner: Inner,
		origin: [isize; DIMS],
		size: [usize; DIMS],
	) -> Grid <Inner, Pos, DIMS> {
		let expected_len = size.into_iter ().product::<usize> ();
		let actual_len = (& inner).storage_len ();
		if expected_len != actual_len {
			panic! ("Expected {} items but was passed {}", expected_len, actual_len);
		}
		Grid { inner, origin, size, phantom: PhantomData }
	}

	pub fn len (& self) -> usize { self.size.into_iter ().product () }
	pub fn size (& self) -> [usize; DIMS] { self.size }

	pub fn origin (& self) -> Pos { Pos::from_scalar (0, self.origin, self.size).unwrap () }
	pub fn peak (& self) -> Pos { Pos::from_scalar (self.len () - 1, self.origin, self.size).unwrap () }

	pub fn get (& self, pos: Pos) -> Option <Inner::Item> {
		Pos::to_scalar (& pos, self.origin, self.size)
			.and_then (|index| (& self.inner).storage_get (index))
	}

	pub fn iter <'a> (& 'a self) -> GridIter <<& 'a Inner as GridStorageIntoIter>::Iter, Pos, DIMS>
			where & 'a Inner: GridStorageIntoIter {
		GridIter {
			inner: (& self.inner).storage_iter (),
			idx: 0,
			origin: self.origin,
			size: self.size,
			phantom: PhantomData,
		}
	}

	pub fn values <'a> (& 'a self) -> <& 'a Inner as GridStorageIntoIter>::Iter
			where & 'a Inner: GridStorageIntoIter {
		(& self.inner).storage_iter ()
	}

}

impl <Inner, Pos, const DIMS: usize> Grid <Inner, Pos, DIMS>
	where
		Inner: GridStorageMut + Clone,
		Pos: GridPos <DIMS> {

	pub fn get_ref (& self, pos: Pos) -> Option <& Inner::Item> {
		Pos::to_scalar (& pos, self.origin, self.size)
			.and_then (|index| (& self.inner).storage_ref (index))
	}

	pub fn get_mut (& mut self, pos: Pos) -> Option <& mut Inner::Item> {
		Pos::to_scalar (& pos, self.origin, self.size)
			.and_then (|index| (& mut self.inner).storage_mut (index))
	}

}

/// Iterator returned by [`Grid::iter`].
///
/// This iterator wraps the iterator from an implementation of [`GridStorage`] and maps from `Item`
/// to `([GridPos], Item)`.

pub struct GridIter <Inner, Pos, const DIMS: usize> {
	inner: Inner,
	idx: usize,
	origin: [isize; DIMS],
	size: [usize; DIMS],
	phantom: PhantomData <Pos>,
}

impl <Inner, Pos, const DIMS: usize> Iterator for GridIter <Inner, Pos, DIMS>
	where
		Inner: Iterator,
		Pos: GridPos <DIMS> {
	type Item = (Pos, Inner::Item);
	fn next (& mut self) -> Option <(Pos, Inner::Item)> {
		if let Some (item) = self.inner.next () {
			let idx = self.idx;
			self.idx += 1;
			Some ((Pos::from_scalar (idx, self.origin, self.size).unwrap (), item))
		} else { None }
	}
	fn nth (& mut self, num: usize) -> Option <(Pos, Inner::Item)> {
		if let Some (item) = self.inner.nth (num) {
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
	fn storage_set (& mut self, idx: usize, item: Self::Item) { self.insert (idx, item); }
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
		let y = match self.y.to_usize () { Some (val) => val, _ => return None };
		let x = match self.x.to_usize () { Some (val) => val, _ => return None };
		if y >= size [0] || x >= size [1] { return None }
		Some (y * size [1] + x)
	}
	fn from_scalar (scalar: usize, origin: [isize; 2], size: [usize; 2]) -> Option <Self> {
		if origin != [0, 0] { unimplemented! () }
		let y = match Val::from_usize (scalar / size [1]) { Some (val) => val, _ => return None };
		let x = match Val::from_usize (scalar % size [1]) { Some (val) => val, _ => return None };
		Some (PosYX { y, x })
	}
}

/// Wrapping iterator which clones items.
///
/// We don't use [`Clone`](iter::Clone) from the standard library because it doesn't handle
/// [`Iterator::skip`] the way we would like. Although it says in the documentation that there is
/// no guarantee each element will be processed, it seems like it does so. Instead, we want to
/// completely bypass any elements which aren't required.

pub struct GridStorageClone <Inner> {
	inner: Inner,
}

impl <Inner> GridStorageClone <Inner> {
	fn new (inner: Inner) -> GridStorageClone <Inner> {
		GridStorageClone { inner }
	}
}

impl <'a, Inner, Item> Iterator for GridStorageClone <Inner>
	where
		Inner: Iterator <Item = & 'a Item>,
		Item: Clone + 'a {
	type Item = Item;
	fn next (& mut self) -> Option <Item> {
		self.inner.next ().map (|item| item.clone ())
	}
	fn nth (& mut self, num: usize) -> Option <Item> {
		self.inner.nth (num).map (|item| item.clone ())
	}
}
