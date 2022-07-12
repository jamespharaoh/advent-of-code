use super::*;

use pos::PosYX;

pub struct Grid <Val, const DIMS: usize = 2> {
	data: Vec <Val>,
	origin: [isize; DIMS],
	size: [usize; DIMS],
}

impl <Val, const DIMS: usize> Grid <Val, DIMS> {
	pub fn new_with (origin: [isize; DIMS], size: [usize; DIMS], val: Val) -> Grid <Val, DIMS> where Val: Clone {
		let data_len = size.into_iter ().product ();
		let data: Vec <Val> = iter::from_fn (|| Some (val.clone ())).take (data_len).collect ();
		if data.len () != data_len { panic! () }
		Grid { data, origin, size }
	}
	pub fn new_from <DataIter> (
		origin: [isize; DIMS],
		size: [usize; DIMS],
		data: DataIter,
	) -> Grid <Val, DIMS>
			where DataIter: IntoIterator <Item = Val> {
		let data_len = size.into_iter ().product ();
		let data: Vec <_> = data.into_iter ().collect ();
		if data.len () != data_len { panic! () }
		Grid { data, origin, size }
	}
	pub fn len (& self) -> usize { self.size.into_iter ().product () }
	pub fn size (& self) -> [usize; DIMS] { self.size }
	pub fn get <Pos: GridPos <DIMS>> (& self, pos: Pos) -> Option <& Val> {
		Pos::to_scalar (& pos, self.origin, self.size).map (|index| & self.data [index])
	}
	pub fn get_mut <Pos: GridPos <DIMS>> (& mut self, pos: Pos) -> Option <& mut Val> {
		Pos::to_scalar (& pos, self.origin, self.size).map (|index| & mut self.data [index])
	}
}

pub trait GridPos <const DIMS: usize>: Sized {
	fn to_scalar (& self, origin: [isize; DIMS], size: [usize; DIMS]) -> Option <usize>;
	fn from_scalar (scalar: usize, origin: [isize; DIMS], size: [usize; DIMS]) -> Option <Self>;
}

impl <Val: num::PrimInt> GridPos <2> for PosYX <Val> {
	fn to_scalar (& self, origin: [isize; 2], size: [usize; 2]) -> Option <usize> {
		if origin != [0, 0] { unimplemented! () }
		let y = match self.y.to_usize () { Some (val) => val, _ => return None };
		let x = match self.x.to_usize () { Some (val) => val, _ => return None };
		if y >= size [0] || x >= size [1] { return None }
		Some (y * size [1] + x)
	}
	fn from_scalar (scalar: usize, origin: [isize; 2], size: [usize; 2]) -> Option <Self> {
		if origin != [0, 0] { unimplemented! () }
		let y = match Val::from (scalar / size [1]) { Some (val) => val, _ => return None };
		let x = match Val::from (scalar % size [1]) { Some (val) => val, _ => return None };
		Some (PosYX { y, x })
	}
}
