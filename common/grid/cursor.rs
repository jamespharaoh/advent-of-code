use super::*;

pub struct GridCursor <'sto, Storage, Pos, const DIMS: usize = 2>
		where Pos: Copy {
	grid: & 'sto Grid <Storage, Pos, DIMS>,
	pos: [usize; DIMS],
	idx: usize,
	phantom: PhantomData <Pos>,
}

impl <'grid, Storage, Pos, const DIMS: usize> GridCursor <'grid, Storage, Pos, DIMS>
	where
		Pos: GridPos <DIMS>,
		Storage: GridStorage + Clone {

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
	pub fn try_add (& self, offset: GridOffset <DIMS>) -> Option <Self>
			where Pos: TryAdd <Pos, Output = Pos> {
		let mut pos = [0; DIMS];
		for dim_idx in 0 .. DIMS {
			let dim_offset = offset.native () [dim_idx];
			let dim_val = self.pos [dim_idx];
			pos [dim_idx] = if 0 <= dim_offset {
				let dim_val = dim_val + dim_offset.unsigned_abs ();
				if self.grid.size [dim_idx] <= dim_val { return None }
				dim_val
			} else {
				if dim_val < dim_offset.unsigned_abs () { return None }
				dim_val - dim_offset.unsigned_abs ()
			};
		}
		let idx = if 0 <= offset.idx () {
			self.idx + offset.idx ().unsigned_abs ()
		} else {
			self.idx - offset.idx ().unsigned_abs ()
		};
		Some (Self {
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

impl <'sto, Storage, Pos, const DIMS: usize> Clone for GridCursor <'sto, Storage, Pos, DIMS>
		where Pos: Copy {
	#[ inline ]
	fn clone (& self) -> Self {
		Self {
			grid: self.grid,
			pos: self.pos,
			idx: self.idx,
			phantom: PhantomData,
		}
	}
}

impl <'sto, Storage, Pos, const DIMS: usize> Copy for GridCursor <'sto, Storage, Pos, DIMS>
	where Pos: Copy {
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
			phantom: PhantomData,
		})
	}

}

#[ derive (Clone, Copy, Debug) ]
pub struct GridOffset <const DIMS: usize> {
	native: [isize; DIMS],
	idx: isize,
}

impl <const DIMS: usize> GridOffset <DIMS> {

	#[ inline ]
	pub fn new <Pos> (size: [usize; DIMS], pos: Pos) -> Self
			where Pos: GridPos <DIMS> {
		Self {
			native: pos.to_native_offset ().unwrap (),
			idx: pos.to_scalar_offset (size).unwrap_or (0),
		}
	}

	#[ inline ]
	#[ must_use ]
	pub const fn native (& self) -> [isize; DIMS] {
		self.native
	}

	#[ inline ]
	#[ must_use ]
	pub const fn idx (& self) -> isize {
		self.idx
	}

}
