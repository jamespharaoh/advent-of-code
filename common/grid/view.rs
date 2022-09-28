use super::*;

pub trait GridView <Pos, const DIMS: usize>: Copy + Sized
	where Pos: GridPos <DIMS> {

	type Item;
	type Cursors: Iterator <Item = GridCursor <Self, Pos, DIMS>>;

	fn origin (self) -> Pos;
	fn size (self) -> Pos;
	fn get_trusted (self, native: Pos, idx: usize) -> Self::Item;
	fn cursors (self) -> Self::Cursors;

	#[ inline ]
	fn len (self) -> usize {
		self.size ().to_array ().into_iter ().map (Pos::Coord::qck_usize).product ()
	}

	#[ inline ]
	fn is_empty (self) -> bool {
		self.size ().to_array ().into_iter ().any (|dim| dim == Pos::Coord::ZERO)
	}

	#[ inline ]
	fn keys (self) -> GridKeysIter <Pos, DIMS> {
		GridKeysIter::new (self.origin (), self.size ())
	}

	#[ inline ]
	fn first_key (self) -> Pos {
		let native = Pos::from_array ([Pos::Coord::ZERO; DIMS]);
		Pos::from_native (native, self.origin ()).unwrap ()
	}

	#[ inline ]
	fn last_key (self) -> Pos {
		let native = Pos::from_array (self.size ().to_array ().map (|val| val - Pos::Coord::ONE));
		Pos::from_native (native, self.origin ()).unwrap ()
	}

	#[ inline ]
	fn get (self, pos: Pos) -> Option <Self::Item> {
		let native = pos.to_native (self.origin ()) ?;
		let idx = native.native_to_index (self.size ()) ?;
		Some (self.get_trusted (native, idx.qck_usize ()))
	}

	#[ inline ]
	fn get_native (self, native: Pos) -> Option <Self::Item> {
		let idx = native.native_to_index (self.size ()) ?;
		Some (self.get_trusted (native, idx.qck_usize ()))
	}

	#[ inline ]
	fn offset (self, pos: Pos) -> NumResult <GridOffset <Pos, DIMS>> {
		GridOffset::new (self.size (), pos)
	}

	#[ inline ]
	fn cursor (self, pos: Pos) -> Option <GridCursor <Self, Pos, DIMS>> {
		let native = pos.to_native (self.origin ()) ?;
		let idx = native.native_to_index (self.size ()) ?;
		Some (GridCursor::new (self, native, idx.qck_usize ()))
	}

	#[ inline ]
	#[ must_use ]
	fn map <Storage, MapFn, Output> (self, map_fn: MapFn) -> GridBuf <Storage, Pos, DIMS>
		where
			MapFn: FnMut (GridCursor <Self, Pos, DIMS>) -> Output,
			Storage: Clone + GridStorage + FromIterator <Output> {
		GridBuf::wrap (
			self.cursors ().map (map_fn).collect (),
			self.origin (),
			self.size ())
	}

	#[ inline ]
	fn extend_map <Storage, MapFn, Output> (
		self,
		amts: [(Pos::Coord, Pos::Coord); DIMS],
		map_fn: MapFn,
	) -> NumResult <GridBuf <Storage, Pos, DIMS>>
		where
			MapFn: FnMut (GridCursor <& GridExtend <Self, Pos, DIMS>, Pos, DIMS>) -> Output,
			Storage: Clone + GridStorage + FromIterator <Output>,
			Self::Item: Default {
		Ok (self.extend (amts) ?.map (map_fn))
	}

	#[ inline ]
	fn try_map <Storage, MapFn, Output, Error> (
		self,
		map_fn: MapFn,
	) -> Result <GridBuf <Storage, Pos, DIMS>, Error>
		where
			MapFn: FnMut (GridCursor <Self, Pos, DIMS>) -> Result <Output, Error>,
			Storage: Clone + GridStorage + FromIterator <Output> {
		Ok (
			GridBuf::wrap (
				self.cursors ().map (map_fn).try_collect () ?,
				self.origin (),
				self.size ())
		)
	}

	#[ inline ]
	fn transform <Storage> (self, axes: [impl Into <Pos>; DIMS]) -> NumResult <GridBuf <Storage, Pos, DIMS>>
		where
			Storage: Clone + GridStorage + FromIterator <Self::Item>,
			Pos::Coord: IntSigned {
		if self.origin ().to_array () != [Pos::Coord::ZERO; DIMS] { unimplemented! () }
		let offsets: [GridOffset <Pos, DIMS>; DIMS] = axes.into_iter ()
			.map (|axis| self.offset (axis.into ()))
			.try_array () ?;
		let mut start = self.cursor (self.first_key ()).unwrap ();
		for offset in offsets {
			let offset = - offset;
			while start.try_add_assign (offset).is_ok () { }
		}
		let storage =
			GridTransformIter::new ([start; DIMS], offsets)
				.map (|cur| cur.item ())
				.collect ();
		let size = Pos::from_array (offsets
			.map (|offset|
				std::iter::successors (
						Some (start),
						|& cur| cur.try_add (offset).ok ())
					.count ())
			.map (|val| Pos::Coord::from_usize (val).unwrap ()));
		Ok (GridBuf::wrap (
			storage,
			Pos::from_array ([Pos::Coord::ZERO; DIMS]),
			size))
	}

}
