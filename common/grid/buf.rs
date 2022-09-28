use super::*;

/// Multi dimensional abstraction over a single dimensional collection
///
/// This allows a simple one-dimensional collection to be accessed in a multi-dimensional way. It
/// wraps a backing collection which implements [`GridStorage`], and takes indexes which implement
/// [`GridPos`].
///
#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct GridBuf <Storage, Pos, const DIMS: usize>
		where Pos: GridPos <DIMS> {
	storage: Storage,
	origin: Pos,
	size: Pos,
	phantom: PhantomData <Pos>,
}

impl <Storage, Pos, const DIMS: usize> GridBuf <Storage, Pos, DIMS>
	where
		Storage: GridStorage + Clone,
		Pos: GridPos <DIMS> {

	#[ inline ]
	#[ must_use ]
	pub fn new (origin: Pos, size: Pos) -> Self
		where
			Storage: FromIterator <Storage::Item>,
			Storage::Item: Clone + Default {
		Self::wrap (
			std::iter::repeat (default ())
				.take (size.to_array ().into_iter ()
					.map (Pos::Coord::pan_usize)
					.product ())
				.collect::<Storage> (),
			origin,
			size)
	}

	#[ inline ]
	pub fn wrap (storage: Storage, origin: Pos, size: Pos) -> Self {
		assert! (! size.to_array ().into_iter ().any (|val| val < Pos::Coord::ONE),
			"Size must be positive in all dimensions: {:?}", size);
		let expected_len =
			size.to_array ().into_iter ()
				.map (Pos::Coord::pan_usize)
				.product::<usize> ();
		let actual_len = storage.storage_len ();
		assert! (expected_len == actual_len,
			"Expected {} items but was passed {}", expected_len, actual_len);
		Self { storage, origin, size, phantom: PhantomData }
	}

}

impl <Storage, Pos, const DIMS: usize> GridBuf <Storage, Pos, DIMS>
	where
		Storage: GridStorage + Clone,
		Pos: GridPos <DIMS> {

	#[ inline ]
	pub fn set (& mut self, pos: Pos, item: Storage::Item) {
		let native = pos.to_native (self.origin).unwrap ();
		let idx = native.native_to_index (self.size).unwrap ();
		self.storage.storage_set (idx.qck_usize (), item);
	}

	#[ inline ]
	pub fn try_set (& mut self, pos: Pos, item: Storage::Item) -> Option <()> {
		let native = pos.to_native (self.origin) ?;
		let idx = native.native_to_index (self.size) ?;
		self.storage.storage_set (idx.qck_usize (), item);
		Some (())
	}

	#[ inline ]
	pub fn translate (& mut self, offset: Pos) -> NumResult <Self> {
		let offset_arr = offset.to_array ();
		let mut origin_arr = self.origin ().to_array ();
		for dim_idx in 0 .. DIMS { origin_arr [dim_idx] -= offset_arr [dim_idx]; }
		let origin = Pos::from_array (origin_arr);
		if ! Pos::validate_dims (origin, self.size) { return Err (Overflow) }
		Ok (Self::wrap (self.storage.clone (), origin, self.size))
	}

}

impl <Item, Pos, const DIMS: usize> GridBuf <Vec <Item>, Pos, DIMS>
	where
		Item: Default,
		Pos: GridPos <DIMS> {

	#[ inline ]
	pub fn reset (& mut self) {
		for item in self.storage.iter_mut () {
			* item = default ();
		}
	}

}

impl <Storage, Pos, const DIMS: usize> GridBuf <Storage, Pos, DIMS>
	where
		Storage: GridStorageMut + Clone,
		Pos: GridPos <DIMS> {

	#[ inline ]
	pub fn get_ref (& self, pos: Pos) -> Option <& Storage::Item> {
		let native = pos.to_native (self.origin) ?;
		let idx = native.native_to_index (self.size) ?;
		self.storage.storage_ref (idx.qck_usize ())
	}

	#[ inline ]
	pub fn get_mut (& mut self, pos: Pos) -> Option <& mut Storage::Item> {
		let native = pos.to_native (self.origin) ?;
		let idx = native.native_to_index (self.size) ?;
		self.storage.storage_mut (idx.qck_usize ())
	}

	/*
	#[ inline ]
	pub fn cursor_mut (& mut self, pos: Pos) -> Option <GridCursorMut <Storage, Pos, DIMS>> {
		let idx = pos.to_scalar (self.origin, self.size) ?;
		let pos = pos.to_native (self.origin) ?;
		Some (GridCursorMut::new (self, pos, idx))
	}
	*/

}

impl <'grd, Storage, Pos, const DIMS: usize> GridView <Pos, DIMS>
	for & 'grd GridBuf <Storage, Pos, DIMS>
	where
		Pos: GridPos <DIMS>,
		Storage: GridStorage {

	type Item = Storage::Item;
	type Cursors = GridCursorIter <Self, Pos, DIMS>;

	#[ inline ]
	fn origin (self) -> Pos {
		self.origin
	}

	#[ inline ]
	fn size (self) -> Pos {
		self.size
	}

	#[ inline ]
	fn get_trusted (self, _native: Pos, idx: usize) -> Storage::Item {
		self.storage.storage_get (idx).unwrap ()
	}

	#[ inline ]
	fn cursors (self) -> GridCursorIter <Self, Pos, DIMS> {
		GridCursorIter {
			grid: self,
			native: Pos::from_array ([Pos::Coord::ZERO; DIMS]),
			idx: 0,
			done: false,
			phantom: PhantomData,
		}
	}

}

impl <'grd, Storage, Pos, const DIMS: usize> GridViewIter <Pos, DIMS>
	for & 'grd GridBuf <Storage, Pos, DIMS>
	where
		Pos: GridPos <DIMS>,
		Storage: GridStorage,
		& 'grd Storage: GridStorageIntoIter <Item = Self::Item> {

	type Values = <& 'grd Storage as GridStorageIntoIter>::Iter;

	#[ inline ]
	fn values (self) -> Self::Values {
		(& self.storage).storage_iter ()
	}

}
