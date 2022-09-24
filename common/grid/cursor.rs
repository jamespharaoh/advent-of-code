use super::*;

impl <Storage, Pos, const DIMS: usize> Grid <Storage, Pos, DIMS>
	where
		Storage: GridStorage + Clone,
		Pos: GridPos <DIMS> {

	#[ inline ]
	pub fn cursor (& self, pos: Pos) -> Option <GridCursor <Storage, Pos, DIMS>> {
		let idx = pos.to_scalar (self.origin, self.size) ?;
		let pos = pos.to_native (self.origin) ?;
		Some (GridCursor::new (self, pos, idx))
	}

	#[ inline ]
	pub const fn cursors (& self) -> GridCursorIter <Storage, Pos, DIMS> {
		GridCursorIter {
			grid: self,
			pos: [0; DIMS],
			idx: 0,
			done: false,
			phantom: PhantomData,
		}
	}

	#[ inline ]
	pub fn offset (& self, pos: impl Into <Pos>) -> GridOffset <DIMS> {
		GridOffset::new (self.size, pos.into ())
	}

}

pub struct GridCursor <'sto, Storage, Pos, const DIMS: usize = 2> {
	grid: & 'sto Grid <Storage, Pos, DIMS>,
	pos: [usize; DIMS],
	idx: usize,
}

impl <'grid, Storage, Pos, const DIMS: usize> GridCursor <'grid, Storage, Pos, DIMS>
	where
		Pos: GridPos <DIMS>,
		Storage: GridStorage + Clone {

	#[ inline ]
	#[ must_use ]
	pub (crate) const fn new (
		grid: & 'grid Grid <Storage, Pos, DIMS>,
		pos: [usize; DIMS],
		idx: usize,
	) -> Self {
		Self { grid, pos, idx }
	}

	#[ inline ]
	#[ must_use ]
	pub fn pos (& self) -> Pos {
		Pos::from_native (self.pos, self.grid.origin).unwrap ()
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
	pub fn item (& self) -> Storage::Item {
		self.grid.storage.storage_get (self.idx).unwrap ()
	}

	#[ inline ]
	#[ must_use ]
	pub const fn walk (
		self,
		offset: GridOffset <DIMS>,
	) -> GridCursorWalk <'grid, Storage, Pos, DIMS> {
		GridCursorWalk { cur: self, offset, done: false }
	}

}

pub struct GridCursorWalk <'grid, Storage, Pos, const DIMS: usize> {
	cur: GridCursor <'grid, Storage, Pos, DIMS>,
	offset: GridOffset <DIMS>,
	done: bool,
}

impl <'grid, Storage, Pos, const DIMS: usize> Iterator
	for GridCursorWalk <'grid, Storage, Pos, DIMS>
	where
		Pos: GridPos <DIMS>,
		Storage: GridStorage {

	type Item = GridCursor <'grid, Storage, Pos, DIMS>;

	fn next (& mut self) -> Option <GridCursor <'grid, Storage, Pos, DIMS>> {
		if self.done { return None }
		let result = self.cur;
		if self.cur.try_add_assign (self.offset).is_err () { self.done = true; }
		Some (result)
	}

}

impl <'grid, Storage, Pos, const DIMS: usize> Clone for GridCursor <'grid, Storage, Pos, DIMS> {

	#[ inline ]
	fn clone (& self) -> Self {
		* self
	}

}

impl <'grid, Storage, Pos, const DIMS: usize> Copy for GridCursor <'grid, Storage, Pos, DIMS> {
}

impl <'grid, Storage, Pos, const DIMS: usize> TryAdd <GridOffset <DIMS>>
	for GridCursor <'grid, Storage, Pos, DIMS>
	where
		Pos: GridPos <DIMS> /*+ TryAdd <Pos, Output = Pos>*/,
		Storage: GridStorage {

	type Output = Self;

	#[ inline ]
	fn try_add (self, offset: GridOffset <DIMS>) -> NumResult <Self> {
		let mut pos = [0; DIMS];
		for dim_idx in 0 .. DIMS {
			let dim_offset = offset.native () [dim_idx];
			let dim_val = self.pos [dim_idx];
			pos [dim_idx] = if 0 <= dim_offset {
				let dim_val = dim_val + dim_offset.unsigned_abs ();
				if self.grid.size [dim_idx] <= dim_val { return Err (Overflow) }
				dim_val
			} else {
				if dim_val < dim_offset.unsigned_abs () { return Err (Overflow) }
				dim_val - dim_offset.unsigned_abs ()
			};
		}
		let idx = if 0 <= offset.idx () {
			self.idx + offset.idx ().unsigned_abs ()
		} else {
			self.idx - offset.idx ().unsigned_abs ()
		};
		Ok (Self { grid: self.grid, pos, idx })
	}

}

impl <'grid, Storage, Pos, const DIMS: usize> TryAddAssign <GridOffset <DIMS>>
	for GridCursor <'grid, Storage, Pos, DIMS>
	where
		Pos: GridPos <DIMS> /*+ TryAdd <Pos, Output = Pos>*/,
		Storage: GridStorage {

	#[ inline ]
	fn try_add_assign (& mut self, offset: GridOffset <DIMS>) -> NumResult <()> {
		for dim_idx in 0 .. DIMS {
			let dim_offset = offset.native () [dim_idx];
			let dim_val = self.pos [dim_idx];
			self.pos [dim_idx] = if 0 <= dim_offset {
				let dim_val = dim_val + dim_offset.unsigned_abs ();
				if self.grid.size [dim_idx] <= dim_val { return Err (Overflow) }
				dim_val
			} else {
				if dim_val < dim_offset.unsigned_abs () { return Err (Overflow) }
				dim_val - dim_offset.unsigned_abs ()
			};
		}
		if 0 <= offset.idx () {
			self.idx += offset.idx ().unsigned_abs ();
		} else {
			self.idx -= offset.idx ().unsigned_abs ();
		};
		Ok (())
	}

}

impl <'grid, Storage, Pos, const DIMS: usize> Debug for GridCursor <'grid, Storage, Pos, DIMS>
	where Pos: Copy + Debug {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		formatter.debug_struct ("GridCursor")
			.field ("pos", & self.pos)
			.field ("idx", & self.idx)
			.finish ()
	}

}

pub struct GridCursorIter <'sto, Storage, Pos, const DIMS: usize> {
	pub (crate) grid: & 'sto Grid <Storage, Pos, DIMS>,
	pub (crate) pos: [usize; DIMS],
	pub (crate) idx: usize,
	pub (crate) done: bool,
	pub (crate) phantom: PhantomData <Pos>,
}

impl <'sto, Storage, Pos, const DIMS: usize> Iterator for GridCursorIter <'sto, Storage, Pos, DIMS>
		where Pos: GridPos <DIMS> + 'sto {
	type Item = GridCursor <'sto, Storage, Pos, DIMS>;

	#[ inline ]
	fn next (& mut self) -> Option <GridCursor <'sto, Storage, Pos, DIMS>> {
		if self.done { return None }
		let cur_pos = self.pos;
		let cur_idx = self.idx;
		for dim_idx in (0 .. DIMS).rev () {
			self.pos [dim_idx] += 1;
			if self.pos [dim_idx] < self.grid.size [dim_idx] { break }
			self.pos [dim_idx] = 0;
			if dim_idx == 0 { self.done = true; }
		}
		self.idx += 1;
		Some (GridCursor {
			grid: self.grid,
			pos: cur_pos,
			idx: cur_idx,
		})
	}

}

pub struct GridCursorMut <'sto, Storage, Pos, const DIMS: usize = 2>
		where Pos: Copy {
	grid: & 'sto mut Grid <Storage, Pos, DIMS>,
	pos: [usize; DIMS],
	idx: usize,
	phantom: PhantomData <Pos>,
}

impl <'grid, Storage, Pos, const DIMS: usize> GridCursorMut <'grid, Storage, Pos, DIMS>
	where
		Pos: GridPos <DIMS>,
		Storage: GridStorageMut + Clone {

	pub (crate) fn new (grid: & 'grid mut Grid <Storage, Pos, DIMS>, pos: [usize; DIMS], idx: usize) -> Self {
		Self { grid, pos, idx, phantom: PhantomData }
	}

	#[ inline ]
	#[ must_use ]
	pub fn pos (& self) -> Pos {
		Pos::from_native (self.pos, self.grid.origin).unwrap ()
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

#[ derive (Clone, Copy, Debug) ]
pub struct GridOffset <const DIMS: usize> {
	pos: [isize; DIMS],
	idx: isize,
}

impl <const DIMS: usize> GridOffset <DIMS> {

	#[ inline ]
	pub fn new <Pos> (size: [usize; DIMS], pos: Pos) -> Self
			where Pos: GridPos <DIMS> {
		Self {
			pos: pos.to_native_offset ().unwrap (),
			idx: pos.to_scalar_offset (size).unwrap_or (0),
		}
	}

	#[ inline ]
	#[ must_use ]
	pub const fn native (& self) -> [isize; DIMS] {
		self.pos
	}

	#[ inline ]
	#[ must_use ]
	pub const fn idx (& self) -> isize {
		self.idx
	}

}

impl <const DIMS: usize> Neg for GridOffset <DIMS> {
	type Output = Self;

	#[ inline ]
	fn neg (self) -> Self {
		Self {
			pos: self.pos.map (|val| - val),
			idx: - self.idx,
		}
	}

}
