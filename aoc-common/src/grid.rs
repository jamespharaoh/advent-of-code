use super::*;

use nums::Int;
use pos::PosYX;

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Grid <Val, Pos, const DIMS: usize = 2> {
	data: Vec <Val>,
	origin: [isize; DIMS],
	size: [usize; DIMS],
	phantom: PhantomData <Pos>,
}

impl <Val, Pos: GridPos <DIMS>, const DIMS: usize> Grid <Val, Pos, DIMS> {

	pub fn new_with (
		origin: [isize; DIMS],
		size: [usize; DIMS],
		val: Val,
	) -> Grid <Val, Pos, DIMS> where Val: Clone {
		let data_len = size.into_iter ().product ();
		let data: Vec <Val> = iter::from_fn (|| Some (val.clone ())).take (data_len).collect ();
		if data.len () != data_len { panic! ("Expected {} values but got {}", data_len, data.len ()) }
		Grid { data, origin, size, phantom: PhantomData }
	}

	pub fn new_from (
		origin: [isize; DIMS],
		size: [usize; DIMS],
		data: Vec <Val>,
	) -> Grid <Val, Pos, DIMS> {
		let data_len = size.into_iter ().product ();
		if data.len () != data_len { panic! ("Expected {} values but got {}", data_len, data.len ()) }
		Grid { data, origin, size, phantom: PhantomData }
	}

	pub fn len (& self) -> usize { self.size.into_iter ().product () }
	pub fn size (& self) -> [usize; DIMS] { self.size }

	pub fn origin (& self) -> Pos { Pos::from_scalar (0, self.origin, self.size).unwrap () }
	pub fn peak (& self) -> Pos { Pos::from_scalar (self.len () - 1, self.origin, self.size).unwrap () }

	pub fn get (& self, pos: Pos) -> Option <& Val> {
		Pos::to_scalar (& pos, self.origin, self.size).map (|index| & self.data [index])
	}

	pub fn get_mut (& mut self, pos: Pos) -> Option <& mut Val> {
		Pos::to_scalar (& pos, self.origin, self.size).map (|index| & mut self.data [index])
	}

	pub fn set (& mut self, pos: Pos, val: Val) {
		self.data [Pos::to_scalar (& pos, self.origin, self.size).unwrap ()] = val;
	}

	pub fn iter (& self) -> impl Iterator <Item = (Pos, & Val)> {
		self.data.iter ().enumerate ().map (|(idx, val)|
			(Pos::from_scalar (idx, self.origin, self.size).unwrap (), val)
		)
	}

}

pub trait GridPos <const DIMS: usize>: Sized {
	fn to_scalar (& self, origin: [isize; DIMS], size: [usize; DIMS]) -> Option <usize>;
	fn from_scalar (scalar: usize, origin: [isize; DIMS], size: [usize; DIMS]) -> Option <Self>;
}

impl GridPos <2> for (usize, usize) {
	fn to_scalar (& self, origin: [isize; 2], size: [usize; 2]) -> Option <usize> {
		if origin != [0, 0] { unimplemented! () }
		usize::checked_add (match usize::checked_mul (self.0, size [1]) {
			Some (val) => val, None => return None }, self.1)
	}
	fn from_scalar (scalar: usize, origin: [isize; 2], size: [usize; 2]) -> Option <Self> {
		if origin != [0, 0] { unimplemented! () }
		Some ((scalar / size [1], scalar % size [1]))
	}
}

impl <Val: Int> GridPos <2> for PosYX <Val> {
	fn to_scalar (& self, origin: [isize; 2], size: [usize; 2]) -> Option <usize> {
		if origin != [0, 0] { unimplemented! () }
		let y = match self.y.to_usize () { Some (val) => val, _ => return None };
		let x = match self.x.to_usize () { Some (val) => val, _ => return None };
		if y >= size [0] || x >= size [1] { return None }
		Some (y * size [1] + x)
	}
	fn from_scalar (scalar: usize, origin: [isize; 2], size: [usize; 2]) -> Option <Self> {
		if origin != [0, 0] { unimplemented! () }
		let y = match Val::from_usize (scalar / size [1]) { Some (val) => val, _ => return None };
		let x = match Val::from_usize (scalar % size [1]) { Some (val) => val, _ => return None };
		Some (PosYX { y, x })
	}
}
