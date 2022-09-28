use super::*;

pub trait GridViewIter <Pos, const DIMS: usize>: GridView <Pos, DIMS>
	where Pos: GridPos <DIMS> {

	type Values: Iterator <Item = Self::Item>;

	fn values (self) -> Self::Values;

	#[ inline ]
	fn iter (self) -> GridIter <Self::Values, Pos, DIMS>
			where Self: Copy + Sized {
		Iterator::zip (self.keys (), self.values ())
	}

	#[ inline ]
	#[ must_use ]
	fn to_buf <Storage> (self) -> GridBuf <Storage, Pos, DIMS>
			where Storage: Clone + GridStorage + FromIterator <Self::Item> {
		GridBuf::wrap (
			self.values ().collect (),
			self.origin (),
			self.size ())
	}

}

pub type GridIter <Values, Pos, const DIMS: usize> =
	std::iter::Zip <GridKeysIter <Pos, DIMS>, Values>;

pub struct GridKeysIter <Pos: GridPos <DIMS>, const DIMS: usize> {
	origin: Pos,
	size: Pos,
	native: Pos,
	done: bool,
	phantom: PhantomData <Pos>,
}

impl <Pos: GridPos <DIMS>, const DIMS: usize> GridKeysIter <Pos, DIMS> {
	pub (crate) fn new (origin: Pos, size: Pos) -> Self {
		Self {
			origin,
			size,
			native: Pos::from_array ([Pos::Coord::ZERO; DIMS]),
			done: false,
			phantom: PhantomData,
		}
	}
}

impl <Pos: GridPos <DIMS>, const DIMS: usize> Iterator for GridKeysIter <Pos, DIMS> {

	type Item = Pos;

	#[ inline ]
	fn next (& mut self) -> Option <Pos> {
		if self.done { return None }
		let size_arr = self.size.to_array ();
		let pos = Pos::from_native (self.native, self.origin);
		let mut native_arr = self.native.to_array ();
		for idx in (0 .. DIMS).rev () {
			native_arr [idx] += Pos::Coord::ONE;
			if native_arr [idx] < size_arr [idx] { break }
			native_arr [idx] = Pos::Coord::ZERO;
			if idx == 0 { self.done = true; }
		}
		self.native = Pos::from_array (native_arr);
		pos
	}

}

pub struct GridCursorIter <Grid, Pos, const DIMS: usize> {
	pub (crate) grid: Grid,
	pub (crate) native: Pos,
	pub (crate) idx: usize,
	pub (crate) done: bool,
	pub (crate) phantom: PhantomData <Pos>,
}

impl <Grid, Pos, const DIMS: usize> Iterator
	for GridCursorIter <Grid, Pos, DIMS>
	where
		Grid: Copy + GridView <Pos, DIMS>,
		Pos: GridPos <DIMS> {

	type Item = GridCursor <Grid, Pos, DIMS>;

	#[ inline ]
	fn next (& mut self) -> Option <GridCursor <Grid, Pos, DIMS>> {
		if self.done { return None }
		let size_arr = self.grid.size ().to_array ();
		let cur_native = self.native;
		let cur_idx = self.idx;
		let mut native_arr = self.native.to_array ();
		for dim_idx in (0 .. DIMS).rev () {
			native_arr [dim_idx] += Pos::Coord::ONE;
			if native_arr [dim_idx] < size_arr [dim_idx] { break }
			native_arr [dim_idx] = Pos::Coord::ZERO;
			if dim_idx == 0 { self.done = true; }
		}
		self.native = Pos::from_array (native_arr);
		self.idx += 1;
		Some (GridCursor::new (self.grid, cur_native, cur_idx))
	}

}

/// Wrapping iterator which clones items.
///
/// We don't use [`Cloned`](iter::Cloned) from the standard library because it doesn't handle
/// [`Iterator::skip`] the way we would like. Although it says in the documentation that there is
/// no guarantee each element will be processed, it seems like it does so. Instead, we want to
/// completely bypass any elements which aren't required.
///
#[ derive (Clone) ]
pub struct GridStorageClone <Storage> {
	storage: Storage,
}

impl <Storage> GridStorageClone <Storage> {
	pub (crate) const fn new (storage: Storage) -> Self {
		Self { storage }
	}
}

impl <'sto, Storage, Item> Iterator for GridStorageClone <Storage>
	where
		Storage: Iterator <Item = & 'sto Item>,
		Item: Clone + 'sto {

	type Item = Item;

	#[ inline ]
	fn next (& mut self) -> Option <Item> {
		self.storage.next ().cloned ()
	}

	#[ inline ]
	fn nth (& mut self, num: usize) -> Option <Item> {
		self.storage.nth (num).cloned ()
	}

}
