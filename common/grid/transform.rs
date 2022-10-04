use super::*;

impl <'grd, Storage, Pos, const DIMS: usize> GridBuf <Storage, Pos, DIMS>
	where
		Storage: GridStorage + Clone + FromIterator <<Storage as GridStorage>::Item> + 'grd,
		Storage::Item: Default,
		& 'grd Storage: GridStorageIntoIter,
		Pos: GridPos <DIMS> {

	#[ inline ]
	pub fn resize (& 'grd self, start: Pos, end: Pos) -> NumResult <Self>
			where <Storage as GridStorage>::Item: Default {
		let storage =
			GridKeysIter::new (start, end)
				.map (|pos| self.get (pos).unwrap_or_default ())
				.collect ();
		Self::wrap_range (storage, start, end)
	}

	#[ inline ]
	pub fn extend_in_place (& 'grd self, amts: [(Pos::Coord, Pos::Coord); DIMS]) -> NumResult <Self>
			where <Storage as GridStorage>::Item: Default {
		let mut start_arr = self.start ().to_array ();
		let mut end_arr = self.end ().to_array ();
		for dim_idx in 0 .. DIMS {
			chk! (start_arr [dim_idx] -= amts [dim_idx].0) ?;
			chk! (end_arr [dim_idx] += amts [dim_idx].1) ?;
		}
		self.resize (Pos::from_array (start_arr), Pos::from_array (end_arr))
	}

}

pub struct GridTransformIter <Pos, const DIMS: usize>
		where Pos: GridPos <DIMS> {
	cursors: [GridCursor <Pos, DIMS>; DIMS],
	offsets: [GridOffset <Pos, DIMS>; DIMS],
	done: bool,
}

impl <Pos, const DIMS: usize> GridTransformIter <Pos, DIMS>
	where Pos: GridPos <DIMS> {

	#[ inline ]
	pub const fn new (
		cursor: GridCursor <Pos, DIMS>,
		offsets: [GridOffset <Pos, DIMS>; DIMS],
	) -> Self {
		let cursors = [cursor; DIMS];
		Self { cursors, offsets, done: false }
	}

}

impl <Pos, const DIMS: usize> Iterator
	for GridTransformIter <Pos, DIMS>
	where Pos: GridPos <DIMS> {

	type Item = GridCursor <Pos, DIMS>;

	#[ inline ]
	fn next (& mut self) -> Option <GridCursor <Pos, DIMS>> {
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
