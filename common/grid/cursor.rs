use super::*;

#[ derive (Clone, Copy) ]
pub struct GridCursor <Grid, Pos, const DIMS: usize> {
	grid: Grid,
	native: Pos,
	idx: usize,
}

impl <Grid, Pos, const DIMS: usize> GridCursor <Grid, Pos, DIMS>
	where Grid: GridView <Pos, DIMS>, Pos: GridPos <DIMS> {

	#[ inline ]
	#[ must_use ]
	pub (crate) const fn new (grid: Grid, native: Pos, idx: usize) -> Self {
		Self { grid, native, idx }
	}

	#[ inline ]
	#[ must_use ]
	pub fn pos (& self) -> Pos {
		Pos::from_native (self.native, self.grid.origin ()).unwrap ()
	}

	#[ inline ]
	#[ must_use ]
	pub const fn native (& self) -> Pos {
		self.native
	}

	#[ inline ]
	#[ must_use ]
	pub const fn index (& self) -> usize {
		self.idx
	}

	#[ inline ]
	#[ must_use ]
	pub fn item (& self) -> Grid::Item {
		self.grid.get_trusted (self.native, self.idx)
	}

	#[ inline ]
	#[ must_use ]
	pub const fn walk (
		self,
		offset: GridOffset <Pos, DIMS>,
	) -> GridCursorWalk <Grid, Pos, DIMS> {
		GridCursorWalk { cur: self, offset, done: false }
	}

}

pub struct GridCursorWalk <Grid, Pos, const DIMS: usize>
		where Pos: GridPos <DIMS> {
	cur: GridCursor <Grid, Pos, DIMS>,
	offset: GridOffset <Pos, DIMS>,
	done: bool,
}

impl <Grid, Pos, const DIMS: usize> Iterator
	for GridCursorWalk <Grid, Pos, DIMS>
	where
		Grid: Copy + GridView <Pos, DIMS>,
		Pos: GridPos <DIMS> {

	type Item = GridCursor <Grid, Pos, DIMS>;

	#[ inline ]
	fn next (& mut self) -> Option <GridCursor <Grid, Pos, DIMS>> {
		if self.done { return None }
		let result = self.cur;
		if self.cur.try_add_assign (self.offset).is_err () { self.done = true; }
		Some (result)
	}

}

impl <Grid, Pos, const DIMS: usize> TryAdd <GridOffset <Pos, DIMS>>
	for GridCursor <Grid, Pos, DIMS>
	where
		Grid: GridView <Pos, DIMS>,
		Pos: GridPos <DIMS> {

	type Output = Self;

	#[ inline ]
	fn try_add (self, offset: GridOffset <Pos, DIMS>) -> NumResult <Self> {
		let size_arr = self.grid.size ().to_array ();
		let offset_arr = offset.val ().to_array ();
		let mut native_arr = self.native.to_array ();
		for dim_idx in 0 .. DIMS {
			let native_val = chk! (native_arr [dim_idx] + offset_arr [dim_idx]) ?;
			if ! (Pos::Coord::ZERO .. size_arr [dim_idx]).contains (& native_val) {
				return Err (Overflow);
			}
			native_arr [dim_idx] = native_val;
		}
		let native = Pos::from_array (native_arr);
		let idx = (self.idx.qck_isize () + offset.idx ()).qck_usize ();
		Ok (Self { grid: self.grid, native, idx })
	}

}

impl <Grid, Pos, const DIMS: usize> TryAddAssign <GridOffset <Pos, DIMS>>
	for GridCursor <Grid, Pos, DIMS>
	where
		Grid: GridView <Pos, DIMS>,
		Pos: GridPos <DIMS> {

	#[ inline ]
	fn try_add_assign (& mut self, offset: GridOffset <Pos, DIMS>) -> NumResult <()> {
		let size_arr = self.grid.size ().to_array ();
		let offset_arr = offset.val ().to_array ();
		let mut native_arr = self.native.to_array ();
		for dim_idx in 0 .. DIMS {
			let native_val = chk! (native_arr [dim_idx] + offset_arr [dim_idx]) ?;
			if ! (Pos::Coord::ZERO .. size_arr [dim_idx]).contains (& native_val) {
				return Err (Overflow);
			}
			native_arr [dim_idx] = native_val;
		}
		self.native = Pos::from_array (native_arr);
		self.idx = (self.idx.qck_isize () + offset.idx ()).qck_usize ();
		Ok (())
	}

}

impl <Grid, Pos, const DIMS: usize> Debug for GridCursor <Grid, Pos, DIMS>
	where
		Grid: GridView <Pos, DIMS>,
		Pos: GridPos <DIMS> {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		formatter.debug_struct ("GridCursor")
			.field ("native", & self.native)
			.field ("idx", & self.idx)
			.finish ()
	}

}

/*
pub struct GridCursorMut <'sto, Storage, Pos, const DIMS: usize = 2>
		where Pos: Copy {
	grid: & 'sto mut GridBuf <Storage, Pos, DIMS>,
	pos: [usize; DIMS],
	idx: usize,
	phantom: PhantomData <Pos>,
}

impl <'grid, Storage, Pos, const DIMS: usize> GridCursorMut <'grid, Storage, Pos, DIMS>
	where
		Pos: GridPos <DIMS>,
		Storage: GridStorageMut + Clone {

	pub (crate) fn new (grid: & 'grid mut GridBuf <Storage, Pos, DIMS>, pos: [usize; DIMS], idx: usize) -> Self {
		Self { grid, pos, idx, phantom: PhantomData }
	}

	#[ inline ]
	#[ must_use ]
	pub fn pos (& self) -> Pos {
		Pos::from_native (self.pos, self.grid.native_origin ()).unwrap ()
	}

	#[ inline ]
	#[ must_use ]
	pub const fn native (& self) -> [usize; DIMS] {
		self.pos
	}

	#[ inline ]
	#[ must_use ]
	pub const fn index (& self) -> usize {
		self.idx
	}

	#[ inline ]
	#[ must_use ]
	pub fn item (& mut self) -> & mut Storage::Item {
		self.grid.storage.storage_mut (self.idx).unwrap ()
	}

	#[ inline ]
	pub fn try_add (self, offset: GridOffset <DIMS>) -> Result <Self, Self>
			where Pos: TryAdd <Pos, Output = Pos> {
		let mut pos = [0; DIMS];
		for dim_idx in 0 .. DIMS {
			let dim_offset = offset.native () [dim_idx];
			let dim_val = self.pos [dim_idx];
			pos [dim_idx] = if 0 <= dim_offset {
				let dim_val = dim_val + dim_offset.unsigned_abs ();
				if self.grid.size [dim_idx] <= dim_val { return Err (self) }
				dim_val
			} else {
				if dim_val < dim_offset.unsigned_abs () { return Err (self) }
				dim_val - dim_offset.unsigned_abs ()
			};
		}
		let idx = if 0 <= offset.idx () {
			self.idx + offset.idx ().unsigned_abs ()
		} else {
			self.idx - offset.idx ().unsigned_abs ()
		};
		Ok (Self {
			grid: self.grid,
			pos,
			idx,
			phantom: PhantomData,
		})
	}

	#[ inline ]
	pub fn try_add_assign (& mut self, offset: & GridOffset <DIMS>) -> Option <()>
			where Pos: TryAdd <Pos, Output = Pos> {
		for dim_idx in 0 .. DIMS {
			let dim_offset = offset.native () [dim_idx];
			let dim_val = self.pos [dim_idx];
			self.pos [dim_idx] = if 0 <= dim_offset {
				let dim_val = dim_val + dim_offset.unsigned_abs ();
				if self.grid.size [dim_idx] <= dim_val { return None }
				dim_val
			} else {
				if dim_val < dim_offset.unsigned_abs () { return None }
				dim_val - dim_offset.unsigned_abs ()
			};
		}
		if 0 <= offset.idx () {
			self.idx += offset.idx ().unsigned_abs ();
		} else {
			self.idx -= offset.idx ().unsigned_abs ();
		};
		Some (())
	}

}

impl <'grid, Storage, Pos, const DIMS: usize> Debug for GridCursorMut <'grid, Storage, Pos, DIMS>
	where Pos: Copy + Debug {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		formatter.debug_struct ("GridCursorMut")
			.field ("pos", & self.pos)
			.field ("idx", & self.idx)
			.finish ()
	}

}
*/

#[ derive (Clone, Copy, Debug, Default) ]
pub struct GridOffset <Pos, const DIMS: usize>
		where Pos: GridPos <DIMS> {
	pos: Pos,
	idx: isize,
}

impl <Pos, const DIMS: usize> GridOffset <Pos, DIMS>
	where Pos: GridPos <DIMS> {

	#[ inline ]
	pub fn new (size: Pos, pos: Pos) -> NumResult <Self> {
		let idx = pos.native_to_index (size).ok_or (Overflow) ?;
		Ok (Self { pos, idx })
	}

	#[ inline ]
	#[ must_use ]
	pub const fn val (& self) -> Pos {
		self.pos
	}

	#[ inline ]
	#[ must_use ]
	pub const fn idx (& self) -> isize {
		self.idx
	}

}

impl <Pos, const DIMS: usize> Neg
	for GridOffset <Pos, DIMS>
	where Pos: GridPos <DIMS>, Pos::Coord: Neg <Output = Pos::Coord> {

	type Output = Self;

	#[ inline ]
	fn neg (self) -> Self {
		Self {
			pos: Pos::from_array (self.pos.to_array ().map (|val| - val)),
			idx: - self.idx,
		}
	}

}
