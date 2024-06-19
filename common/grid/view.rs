use super::*;

pub trait GridView <Pos, const DIMS: usize>: Copy + Sized
	where Pos: GridPos <DIMS> {

	type Item;
	type Cursors: Iterator <Item = GridCursor <Pos, DIMS>>;

	fn start (self) -> Pos;
	fn end (self) -> Pos;
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
		GridKeysIter::new (self.start (), self.end ())
	}

	#[ inline ]
	fn first_key (self) -> Pos {
		self.start ()
	}

	#[ inline ]
	fn last_key (self) -> Pos {
		let mut last_arr = self.end ().to_array ();
		for dim_idx in 0 .. DIMS { last_arr [dim_idx] -= Pos::Coord::ONE; }
		Pos::from_array (last_arr)
	}

	#[ inline ]
	fn get (self, pos: Pos) -> Option <Self::Item> {
		let native = pos.to_native (self.start ()) ?;
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
	fn cursor (self, pos: Pos) -> Option <GridCursor <Pos, DIMS>> {
		let native = pos.to_native (self.start ()) ?;
		let idx = native.native_to_index (self.size ()) ?;
		Some (GridCursor::new_grid (self, native, idx.qck_usize ()))
	}

	#[ inline ]
	#[ must_use ]
	fn map <Storage, MapFn, Output> (self, map_fn: MapFn) -> GridBuf <Storage, Pos, DIMS>
		where
			MapFn: FnMut (GridCursor <Pos, DIMS>) -> Output,
			Storage: Clone + GridStorage + FromIterator <Output> {
		let storage = self.cursors ().map (map_fn).collect ();
		GridBuf::wrap_range (storage, self.start (), self.end ()).unwrap ()
	}

	#[ inline ]
	fn extend_map <Storage, MapFn, Output> (
		self,
		amts: [(Pos::Coord, Pos::Coord); DIMS],
		map_fn: MapFn,
	) -> NumResult <GridBuf <Storage, Pos, DIMS>>
		where
			MapFn: FnMut (GridCursor <Pos, DIMS>) -> Output,
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
			MapFn: FnMut (GridCursor <Pos, DIMS>) -> Result <Output, Error>,
			Storage: Clone + GridStorage + FromIterator <Output> {
		let storage = self.cursors ().map (map_fn).try_collect () ?;
		Ok (GridBuf::wrap_range (storage, self.start (), self.end ()).unwrap ())
	}

	#[ inline ]
	fn transform <Storage> (
		self,
		start: Pos,
		axes: [impl Into <Pos>; DIMS],
	) -> NumResult <GridBuf <Storage, Pos, DIMS>>
		where
			Storage: Clone + GridStorage + FromIterator <Self::Item>,
			Pos::Coord: IntSigned {
		if self.start () != Pos::default () { unimplemented! () }
		let offsets: [GridOffset <Pos, DIMS>; DIMS] = axes.into_iter ()
			.map (|axis| self.offset (axis.into ()))
			.try_array () ?;
		let mut cur = self.cursor (self.first_key ()).unwrap ();
		for offset in offsets {
			let offset = - offset;
			while cur.try_add_assign (offset).is_ok () { }
		}
		let storage =
			GridTransformIter::new (cur, offsets)
				.map (|cur| cur.get (self))
				.collect ();
		let size = Pos::from_array (offsets
			.map (|offset|
				std::iter::successors (
						Some (cur),
						|& cur| cur.try_add (offset).ok ())
					.count ())
			.map (|val| Pos::Coord::from_usize (val).unwrap ()));
		let end = Pos::from_array (
			std::iter::zip (start.to_array (), size.to_array ())
				.map (|(start, size)| chk! (start + size))
				.try_array () ?);
		GridBuf::wrap_range (storage, start, end)
	}

	#[ inline ]
	fn contains (self, pos: Pos) -> bool {
		pos.to_array ().into_iter ()
			.zip (self.start ().to_array ())
			.zip (self.end ().to_array ())
			.map (|((val, start), end)| (val, start, end))
			.all (|(val, start, end)| start <= val && val <= end)
	}

}
