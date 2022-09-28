use super::*;

impl <'grd, Storage, Pos, const DIMS: usize> GridBuf <Storage, Pos, DIMS>
	where
		Storage: GridStorage + Clone + FromIterator <<Storage as GridStorage>::Item> + 'grd,
		Storage::Item: Default,
		& 'grd Storage: GridStorageIntoIter,
		Pos: GridPos <DIMS> {

	#[ inline ]
	pub fn resize (& 'grd self, origin: Pos, size: Pos) -> NumResult <Self>
			where <Storage as GridStorage>::Item: Default {
		if ! Pos::validate_dims (origin, size) { return Err (Overflow) }
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
	pub fn extend_in_place (& 'grd self, amts: [(Pos::Coord, Pos::Coord); DIMS]) -> NumResult <Self>
			where <Storage as GridStorage>::Item: Default {
		let origin_arr = self.origin ().to_array ();
		let size_arr = self.size ().to_array ();
		self.resize (
			Pos::from_array (array::from_fn (|idx| origin_arr [idx] + amts [idx].0)),
			Pos::from_array (array::from_fn (|idx| size_arr [idx] + amts [idx].0 + amts [idx].1)))
	}

}

pub struct GridTransformIter <Grid, Pos, const DIMS: usize>
		where Pos: GridPos <DIMS> {
	cursors: [GridCursor <Grid, Pos, DIMS>; DIMS],
	offsets: [GridOffset <Pos, DIMS>; DIMS],
	done: bool,
}

impl <Grid, Pos, const DIMS: usize> GridTransformIter <Grid, Pos, DIMS>
	where Pos: GridPos <DIMS> {

	#[ inline ]
	pub const fn new (
		cursors: [GridCursor <Grid, Pos, DIMS>; DIMS],
		offsets: [GridOffset <Pos, DIMS>; DIMS],
	) -> Self {
		Self { cursors, offsets, done: false }
	}

}

impl <Grid, Pos, const DIMS: usize> Iterator
	for GridTransformIter <Grid, Pos, DIMS>
	where
		Grid: GridView <Pos, DIMS>,
		Pos: GridPos <DIMS> {

	type Item = GridCursor <Grid, Pos, DIMS>;

	#[ inline ]
	fn next (& mut self) -> Option <GridCursor <Grid, Pos, DIMS>> {
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
