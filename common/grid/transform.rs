use super::*;

impl <Storage, Pos, const DIMS: usize> Grid <Storage, Pos, DIMS>
	where
		Storage: GridStorage + Clone + FromIterator <Storage::Item>,
		Pos: GridPos <DIMS> {

	#[ inline ]
	pub fn resize (& self, origin: [isize; DIMS], size: [usize; DIMS]) -> NumResult <Self>
			where Storage::Item: Default {
		if ! Self::validate_dims (origin, size) { return Err (Overflow) }
		Ok (
			Self::wrap (
				GridKeysIter::new (origin, size)
					.map (|pos| self.get (pos).unwrap_or_default ())
					.collect (),
				origin,
				size)
		)
	}

	#[ inline ]
	pub fn extend (& self, amts: [(isize, isize); DIMS]) -> NumResult <Self>
			where Storage::Item: Default {
		self.resize (
			array::from_fn (|idx| self.origin [idx] + amts [idx].0),
			array::from_fn (|idx| (self.size [idx].as_isize () + amts [idx].0 + amts [idx].1).as_usize ()))
	}

	#[ inline ]
	pub fn translate (& mut self, offset: Pos) -> NumResult <Self> {
		let offset = offset.to_native_offset ().unwrap ();
		let origin = array::from_fn (|dim_idx| self.origin [dim_idx] - offset [dim_idx]);
		if ! Self::validate_dims (origin, self.size) { return Err (Overflow) }
		Ok (Self::wrap (self.storage.clone (), origin, self.size))
	}

	#[ inline ]
	#[ must_use ]
	pub fn transform (& self, axes: [impl Into <Pos>; DIMS]) -> Self {
		if self.origin != [0; DIMS] { unimplemented! () }
		let offsets = axes.map (|axis| self.offset (axis.into ()));
		let mut start = self.cursor (self.first_key ()).unwrap ();
		for offset in offsets {
			let offset = - offset;
			while start.try_add_assign (offset).is_ok () { }
		}
		let storage =
			GridTransformIter { cursors: [start; DIMS], offsets, done: false }
				.map (|cur| cur.item ())
				.collect ();
		let size = offsets.map (|offset|
			std::iter::successors (
					Some (start),
					|& cur| cur.try_add (offset).ok ())
				.count ());
		Self::wrap (storage, [0; DIMS], size)
	}

}

struct GridTransformIter <'grid, Storage, Pos, const DIMS: usize> {
	cursors: [GridCursor <'grid, Storage, Pos, DIMS>; DIMS],
	offsets: [GridOffset <DIMS>; DIMS],
	done: bool,
}

impl <'grid, Storage, Pos, const DIMS: usize> Iterator
	for GridTransformIter <'grid, Storage, Pos, DIMS>
	where
		Pos: GridPos <DIMS>,
		Storage: GridStorage {

	type Item = GridCursor <'grid, Storage, Pos, DIMS>;

	fn next (& mut self) -> Option <GridCursor <'grid, Storage, Pos, DIMS>> {
		if self.done { return None }
		let result = self.cursors [DIMS - 1];
		for idx_0 in (0 .. DIMS).rev () {
			if let Ok (cur) = self.cursors [idx_0].try_add (self.offsets [idx_0]) {
				for idx_1 in idx_0 .. DIMS { self.cursors [idx_1] = cur; }
				return Some (result);
			}
		}
		self.done = true;
		Some (result)
	}

}
