use super::*;

/// Multi dimensional abstraction over a single dimensional collection
///
/// This allows a simple one-dimensional collection to be accessed in a multi-dimensional way. It
/// wraps a backing collection which implements [`GridStorage`], and takes indexes which implement
/// [`GridPos`].
///
#[ derive (Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct GridBuf <Storage, Pos, const DIMS: usize>
		where Pos: GridPos <DIMS> {
	storage: Storage,
	start: Pos,
	end: Pos,
	size: Pos,
	phantom: PhantomData <Pos>,
}

impl <Storage, Pos, const DIMS: usize> GridBuf <Storage, Pos, DIMS>
	where
		Storage: GridStorage + Clone,
		Pos: GridPos <DIMS> {

	#[ inline ]
	pub fn wrap_range (storage: Storage, start: Pos, end: Pos) -> NumResult <Self> {
		assert! (
			std::iter::zip (start.to_array (), end.to_array ())
				.all (|(start, end)| start < end),
			"Size must be positive in all dimensions: {start:?} to {end:?}");
		let size = Pos::from_array (
			std::iter::zip (start.to_array (), end.to_array ())
				.map (|(start, end)| chk! (end - start))
				.try_array () ?);
		let expected_len =
			size.to_array ().into_iter ()
				.map (Pos::Coord::pan_usize)
				.product::<usize> ();
		let actual_len = storage.storage_len ();
		assert! (expected_len == actual_len,
			"Expected {expected_len} items but was passed {actual_len}");
		Ok (Self { storage, start, end, size, phantom: PhantomData })
	}

	#[ inline ]
	pub fn wrap_size (storage: Storage, size: Pos) -> Self {
		Self::wrap_range (storage, Pos::default (), size).unwrap ()
	}

	#[ inline ]
	pub fn new_range (start: Pos, end: Pos) -> NumResult <Self>
		where
			Storage: FromIterator <Storage::Item>,
			Storage::Item: Clone + Default {
		assert! (
			std::iter::zip (start.to_array (), end.to_array ())
				.all (|(start, end)| start < end),
			"Size must be positive in all dimensions: {start:?} to {end:?}");
		let size = Pos::from_array (
			std::iter::zip (start.to_array (), end.to_array ())
				.map (|(start, end)| chk! (end - start))
				.try_array () ?);
		let storage =
			std::iter::repeat (default ())
				.take (size.to_array ().into_iter ()
					.map (Pos::Coord::pan_usize)
					.product ())
				.collect::<Storage> ();
		Ok (Self { storage, start, end, size, phantom: PhantomData })
	}

	#[ inline ]
	pub fn new_range_with (start: Pos, end: Pos, item: Storage::Item) -> NumResult <Self>
		where
			Storage: FromIterator <Storage::Item>,
			Storage::Item: Clone + Default {
		assert! (
			std::iter::zip (start.to_array (), end.to_array ())
				.all (|(start, end)| start < end),
			"Size must be positive in all dimensions: {start:?} to {end:?}");
		let size = Pos::from_array (
			std::iter::zip (start.to_array (), end.to_array ())
				.map (|(start, end)| chk! (end - start))
				.try_array () ?);
		let storage =
			std::iter::repeat (item)
				.take (size.to_array ().into_iter ()
					.map (Pos::Coord::pan_usize)
					.product ())
				.collect::<Storage> ();
		Ok (Self { storage, start, end, size, phantom: PhantomData })
	}

	#[ inline ]
	#[ must_use ]
	pub fn new_size (size: Pos) -> Self
		where
			Storage: FromIterator <Storage::Item>,
			Storage::Item: Clone + Default {
		Self::new_range (Pos::default (), size).unwrap ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn into_storage (self) -> Storage {
		self.storage
	}

}

impl <Storage, Pos, const DIMS: usize> GridBuf <Storage, Pos, DIMS>
	where
		Storage: GridStorage + Clone,
		Pos: GridPos <DIMS> {

	#[ inline ]
	pub fn set_index (& mut self, idx: usize, item: Storage::Item) {
		self.storage.storage_set (idx.qck_usize (), item);
	}

	#[ inline ]
	pub fn set (& mut self, pos: Pos, item: Storage::Item) {
		let native = pos.to_native (self.start).unwrap ();
		let idx = native.native_to_index (self.size).unwrap ();
		self.storage.storage_set (idx.qck_usize (), item);
	}

	#[ inline ]
	#[ must_use ]
	pub fn try_set (& mut self, pos: Pos, item: Storage::Item) -> Option <()> {
		let native = pos.to_native (self.start) ?;
		let idx = native.native_to_index (self.size) ?;
		self.storage.storage_set (idx.qck_usize (), item);
		Some (())
	}

	#[ inline ]
	pub fn translate (& self, offset: Pos) -> NumResult <Self> {
		let offset_arr = offset.to_array ();
		let mut start_arr = self.start.to_array ();
		let mut end_arr = self.end.to_array ();
		for dim_idx in 0 .. DIMS {
			chk! (start_arr [dim_idx] += offset_arr [dim_idx]) ?;
			chk! (end_arr [dim_idx] += offset_arr [dim_idx]) ?;
		}
		let start = Pos::from_array (start_arr);
		let end = Pos::from_array (end_arr);
		Ok (Self {
			storage: self.storage.clone (),
			start,
			end,
			size: self.size,
			phantom: PhantomData,
		})
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
		let native = pos.to_native (self.start) ?;
		let idx = native.native_to_index (self.size) ?;
		self.storage.storage_ref (idx.qck_usize ())
	}

	#[ inline ]
	pub fn get_mut (& mut self, pos: Pos) -> Option <& mut Storage::Item> {
		let native = pos.to_native (self.start) ?;
		let idx = native.native_to_index (self.size) ?;
		self.storage.storage_mut (idx.qck_usize ())
	}

}

impl <Storage, Pos, const DIMS: usize> Debug
	for GridBuf <Storage, Pos, DIMS>
	where Pos: GridPos <DIMS> {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		formatter.debug_struct ("GridBuf")
			.field ("start", & self.start)
			.field ("end", & self.end)
			.field ("size", & self.size)
			.finish ()
	}

}

impl <'grd, Storage, Pos, const DIMS: usize> GridView <Pos, DIMS>
	for & 'grd GridBuf <Storage, Pos, DIMS>
	where
		Pos: GridPos <DIMS>,
		Storage: GridStorage {

	type Item = Storage::Item;
	type Cursors = GridCursorIter <Pos, DIMS>;

	#[ inline ]
	fn start (self) -> Pos {
		self.start
	}

	#[ inline ]
	fn end (self) -> Pos {
		self.end
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
	fn cursors (self) -> GridCursorIter <Pos, DIMS> {
		GridCursorIter::new_grid (self)
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
