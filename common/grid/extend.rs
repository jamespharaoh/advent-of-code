use super::*;

pub trait GridViewExtend <Pos, const DIMS: usize>: GridView <Pos, DIMS>
	where Pos: GridPos <DIMS> {

	#[ inline ]
	fn extend (self, amts: [(Pos::Coord, Pos::Coord); DIMS]) -> NumResult <GridExtend <Self, Pos, DIMS>> {
		let mut start_arr = self.start ().to_array ();
		let mut end_arr = self.end ().to_array ();
		let mut size_arr = self.size ().to_array ();
		for dim_idx in 0 .. DIMS {
			chk! (start_arr [dim_idx] -= amts [dim_idx].0) ?;
			chk! (end_arr [dim_idx] += amts [dim_idx].1) ?;
			chk! (size_arr [dim_idx] += amts [dim_idx].0 + amts [dim_idx].1) ?;
		}
		let start = Pos::from_array (start_arr);
		let end = Pos::from_array (end_arr);
		let size = Pos::from_array (size_arr);
		let ranges = array::from_fn (|dim_idx|
			(amts [dim_idx].0, size_arr [dim_idx] - amts [dim_idx].1));
		let idx_fix = {
			let self_size_arr = self.size ().to_array ();
			let mut mul = 1_usize;
			let mut sum = 0_usize;
			for (dim_idx, & (start, _)) in amts.iter ().enumerate ().rev () {
				sum += start.pan_usize () * mul;
				mul *= self_size_arr [dim_idx].pan_usize ();
			}
			sum
		};
		Ok (GridExtend { inner: self, start, end, size, ranges, idx_fix })
	}

}

impl <Grid, Pos, const DIMS: usize> GridViewExtend <Pos, DIMS> for Grid
	where
		Grid: GridView <Pos, DIMS>,
		Pos: GridPos <DIMS> {
}

#[ derive (Clone, Copy) ]
pub struct GridExtend <Inner, Pos, const DIMS: usize>
	where
		Inner: GridView <Pos, DIMS>,
		Pos: GridPos <DIMS> {
	inner: Inner,
	start: Pos,
	end: Pos,
	size: Pos,
	ranges: [(Pos::Coord, Pos::Coord); DIMS],
	idx_fix: usize,
}

impl <'grd, Inner, Pos, const DIMS: usize> GridView <Pos, DIMS>
	for & 'grd GridExtend <Inner, Pos, DIMS>
	where
		Inner: GridView <Pos, DIMS>,
		Inner::Item: Default,
		Pos: GridPos <DIMS> {

	type Item = Inner::Item;
	type Cursors = GridExtendCursors <Pos, DIMS>;

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
	fn get_trusted (self, native: Pos, idx: usize) -> Self::Item {
		let native_arr = native.to_array ();
		let mut inner_native_arr = [Pos::Coord::ZERO; DIMS];
		for dim_idx in 0 .. DIMS {
			let val = native_arr [dim_idx];
			let (rng_start, rng_end) = self.ranges [dim_idx];
			if ! (rng_start .. rng_end).contains (& val) { return default () }
			let inner_val = val - rng_start;
			inner_native_arr [dim_idx] = inner_val;
		}
		let inner_native = Pos::from_array (inner_native_arr);
		let inner_idx = idx - self.idx_fix;
		self.inner.get_trusted (inner_native, inner_idx)
	}

	#[ inline ]
	fn offset (self, pos: Pos) -> NumResult <GridOffset <Pos, DIMS>> {
		GridOffset::new (self.inner.size (), pos)
	}

	#[ inline ]
	fn cursor (self, pos: Pos) -> Option <GridCursor <Pos, DIMS>> {
		let native = pos.to_native (self.start) ?;
		let (idx, _) = native.to_array ().into_iter ()
			.zip (self.inner.size ().to_array ()).rev ()
			.fold ((Pos::Coord::ZERO, Pos::Coord::ONE),
				|(sum, mul), (val, size)| (sum + val * mul, mul * size));
		Some (GridCursor::new_grid (self, native, idx.qck_usize ()))
	}

	#[ inline ]
	fn cursors (self) -> GridExtendCursors <Pos, DIMS>
			where Self: Sized {
		let inner_size_arr = self.inner.size ().to_array ();
		GridExtendCursors {
			start: self.start (),
			size: self.size (),
			native: Pos::from_array ([Pos::Coord::ZERO; DIMS]),
			idx: 0,
			idx_fix: {
				let mut idx_fix = [0; DIMS];
				let mut mul = 1_usize;
				let mut sum = 0_usize;
				for (dim_idx, (& (rng_start, rng_end), & size))
					in self.ranges.iter ()
						.zip (& self.size.to_array ())
						.enumerate ()
						.rev () {
					let val = chk! (size - rng_end + rng_start).unwrap ().pan_usize ();
					sum += chk! (val * mul).unwrap ();
					idx_fix [dim_idx] = chk! (sum * mul).unwrap ();
					mul *= inner_size_arr [dim_idx].pan_usize ();
				}
				idx_fix
			},
			done: false,
		}
	}

}

impl <'grd, Inner, Pos, const DIMS: usize> Debug
	for & 'grd GridExtend <Inner, Pos, DIMS>
	where
		Inner: GridView <Pos, DIMS>,
		Inner::Item: Default,
		Pos: GridPos <DIMS> {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		formatter.debug_struct ("GridExtend")
			.field ("start", & self.start)
			.field ("end", & self.end)
			.field ("size", & self.size)
			.field ("ranges", & self.ranges)
			.field ("idx_fix", & self.idx_fix)
			.finish ()
	}

}

impl <'grd, Inner, Pos, const DIMS: usize> GridViewIter <Pos, DIMS>
	for & 'grd GridExtend <Inner, Pos, DIMS>
	where
		Inner: GridViewIter <Pos, DIMS>,
		Inner::Item: Default,
		Pos: GridPos <DIMS> {

	type Values = GridExtendIter <Inner::Values, Pos, DIMS>;

	#[ inline ]
	fn values (self) -> Self::Values {
		GridExtendIter {
			inner: self.inner.values (),
			native: Pos::from_array ([Pos::Coord::ZERO; DIMS]),
			size: self.size,
			ranges: self.ranges,
			done: false,
		}
	}

}

#[ derive (Clone) ]
pub struct GridExtendIter <Inner, Pos, const DIMS: usize>
		where Pos: GridPos <DIMS> {
	inner: Inner,
	native: Pos,
	size: Pos,
	ranges: [(Pos::Coord, Pos::Coord); DIMS],
	done: bool,
}

impl <Inner, Pos, const DIMS: usize> Iterator
	for GridExtendIter <Inner, Pos, DIMS>
	where
		Inner: Iterator,
		Inner::Item: Default,
		Pos: GridPos <DIMS> {

	type Item = Inner::Item;

	#[ inline ]
	fn next (& mut self) -> Option <Self::Item> {
		if self.done { return None }
		let val = {
			if self.native.to_array ().iter ()
					.zip (& self.ranges)
					.all (|(& val, & (start, end))| (start .. end).contains (& val)) {
				self.inner.next ().unwrap ()
			} else {
				Self::Item::default ()
			}
		};
		let mut native_arr = self.native.to_array ();
		for (dim_idx, (pos, & size))
			in native_arr.iter_mut ()
				.zip (& self.size.to_array ())
				.enumerate ()
				.rev () {
			* pos += Pos::Coord::ONE;
			if * pos < size { break }
			* pos = Pos::Coord::ZERO;
			if dim_idx == 0 { self.done = true; }
		}
		self.native = Pos::from_array (native_arr);
		Some (val)
	}

}

impl <Inner, Pos, const DIMS: usize> FusedIterator
	for GridExtendIter <Inner, Pos, DIMS>
	where
		Inner: Iterator,
		Inner::Item: Default,
		Pos: GridPos <DIMS> {
}

pub struct GridExtendCursors <Pos, const DIMS: usize> {
	start: Pos,
	size: Pos,
	native: Pos,
	idx: usize,
	idx_fix: [usize; DIMS],
	done: bool,
}

impl <Pos, const DIMS: usize> Iterator
	for GridExtendCursors <Pos, DIMS>
	where Pos: GridPos <DIMS> {

	type Item = GridCursor <Pos, DIMS>;

	#[ inline ]
	fn next (& mut self) -> Option <GridCursor <Pos, DIMS>> {
		if self.done { return None }
		let size_arr = self.size.to_array ();
		let cur_native = self.native;
		let cur_idx = self.idx;
		let mut native_arr = cur_native.to_array ();
		for dim_idx in (0 .. DIMS).rev () {
			native_arr [dim_idx] += Pos::Coord::ONE;
			if native_arr [dim_idx] < size_arr [dim_idx] { break }
			native_arr [dim_idx] = Pos::Coord::ZERO;
			if 0 < dim_idx {
				self.idx -= self.idx_fix [dim_idx];
			} else { self.done = true; }
		}
		self.native = Pos::from_array (native_arr);
		self.idx += 1;
		Some (GridCursor::new_size (self.start, self.size, cur_native, cur_idx))
	}

}
