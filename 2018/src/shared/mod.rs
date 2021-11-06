use std::fmt::Debug;
use std::iter;
use std::mem;

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
pub struct Pos {
	pub x: i64,
	pub y: i64
}

impl Pos {

	pub fn new (x: i64, y: i64) -> Pos {
		Pos { x, y }
	}

	pub fn up (self, num: i64) -> Pos {
		Pos::new (self.x, i64::checked_sub (self.y, num).unwrap ())
	}

	pub fn down (self, num: i64) -> Pos {
		Pos::new (self.x, i64::checked_add (self.y, num).unwrap ())
	}

	pub fn left (self, num: i64) -> Pos {
		Pos::new (i64::checked_sub (self.x, num).unwrap (), self.y)
	}

	pub fn right (self, num: i64) -> Pos {
		Pos::new (i64::checked_add (self.x, num).unwrap (), self.y)
	}

	pub fn four_neighbours (& self) -> [Pos; 4] {
		[
			self.up (1),
			self.down (1),
			self.left (1),
			self.right (1),
		]
	}

}

impl From <(i64, i64)> for Pos {
	fn from ((x, y): (i64, i64)) -> Pos {
		Pos { x, y }
	}
}

#[ derive (Clone) ]
pub struct Grid <Value> {
	top: i64,
	bottom: i64,
	left: i64,
	right: i64,
	width: usize,
	height: usize,
	default: Value,
	bounded: bool,
	data: Vec <Value>,
}

impl <Value> Grid <Value> where Value: Clone {

	pub fn builder () -> GridBuilder <Value> {
		GridBuilder {
			top: None,
			bottom: None,
			left: None,
			right: None,
			width: None,
			height: None,
			default: None,
			bounded: None,
		}
	}

	pub fn set (& mut self, pos: Pos, mut value: Value) -> Value {
		let index = match self.index (pos) {
			Some (index) => index,
			None => panic! ("out of bounds"),
		};
		mem::swap (& mut value, & mut self.data [index]);
		value
	}

	pub fn get (& self, pos: Pos) -> Value {
		let index = match self.index (pos) {
			Some (index) => index,
			None => if self.bounded {
				panic! ("out of bounds");
			} else {
				return self.default.clone ();
			},
		};
		self.data [index].clone ()
	}

	pub fn index (& self, pos: Pos) -> Option <usize> {
		if pos.x < self.left || pos.x >= self.right { return None }
		if pos.y < self.top || pos.y >= self.bottom { return None }
		Some ((pos.y - self.top) as usize * self.width + (pos.x - self.left) as usize)
	}

	pub fn posns (& self) -> GridPosIter {
		GridPosIter::new (
			Pos::new (self.left, self.top),
			Pos::new (self.right, self.bottom),
		)
	}

	pub fn values (& self) -> impl Iterator <Item = & Value> {
		self.data.iter ()
	}

}

pub struct GridPosIter {
	pos: Pos,
	bottom: i64,
	left: i64,
	right: i64,
}

impl GridPosIter {

	pub fn new (top_left: Pos, bottom_right: Pos) -> GridPosIter {
		GridPosIter {
			pos: top_left,
			left: top_left.x,
			right: bottom_right.x,
			bottom: bottom_right.y,
		}
	}

}

impl Iterator for GridPosIter {

	type Item = Pos;

	fn next (& mut self) -> Option <Pos> {
		if self.pos.y == self.bottom { return None }
		let result = self.pos;
		self.pos.x += 1;
		if self.pos.x == self.right {
			self.pos.x = self.left;
			self.pos.y += 1;
		}
		Some (result)
	}

}

pub struct GridBuilder <Value> {
	top: Option <i64>,
	bottom: Option <i64>,
	left: Option <i64>,
	right: Option <i64>,
	width: Option <usize>,
	height: Option <usize>,
	default: Option <Value>,
	bounded: Option <bool>,
}

impl <Value> GridBuilder <Value> where Value: Clone {

	/*
	pub fn top (mut self, top: i64) -> GridBuilder <Value> {
		if self.top.is_some () { panic! () }
		self.top = Some (top);
		self
	}

	pub fn bottom (mut self, bottom: i64) -> GridBuilder <Value> {
		if self.top.is_none () { panic! () }
		if self.bottom.is_some () { panic! () }
		if bottom < self.top.unwrap () { panic! () }
		self.bottom = Some (bottom);
		self.height = Some (usize::checked_sub (
			bottom.try_into ().unwrap (),
			self.top.unwrap ().try_into ().unwrap (),
		).unwrap ());
		self
	}

	pub fn left (mut self, left: i64) -> GridBuilder <Value> {
		if self.left.is_some () { panic! () }
		self.left = Some (left);
		self
	}

	pub fn right (mut self, right: i64) -> GridBuilder <Value> {
		if self.left.is_none () { panic! () }
		if self.right.is_some () { panic! () }
		if right < self.left.unwrap () { panic! () }
		self.right = Some (right);
		self.width = Some (usize::checked_sub (
			right.try_into ().unwrap (),
			self.left.unwrap ().try_into ().unwrap (),
		).unwrap ());
		self
	}
	*/

	pub fn width <Num> (mut self, width: Num) -> GridBuilder <Value> where Num: TryInto <usize> {
		let width: usize = width.try_into ().map_err (|_| "overflow").unwrap ();
		if self.left.is_none () { self.left = Some (0) }
		self.right = Some (i64::checked_add (
			self.left.unwrap (),
			width.try_into ().unwrap (),
		).unwrap ());
		self.width = Some (width);
		self
	}

	pub fn height <Num> (mut self, height: Num) -> GridBuilder <Value> where Num: TryInto <usize> {
		let height: usize = height.try_into ().map_err (|_| "overflow").unwrap ();
		if self.top.is_none () { self.top = Some (0) }
		self.bottom = Some (i64::checked_add (
			self.left.unwrap (),
			height.try_into ().unwrap (),
		).unwrap ());
		self.height = Some (height);
		self
	}

	/*
	pub fn bounded (mut self) -> GridBuilder <Value> {
		if self.bounded.is_some () { panic! () }
		self.bounded = Some (true);
		self
	}
	*/

	pub fn unbounded (mut self) -> GridBuilder <Value> {
		if self.bounded.is_some () { panic! () }
		self.bounded = Some (false);
		self
	}

	pub fn default (mut self, default: Value) -> GridBuilder <Value> {
		if self.default.is_some () { panic! () }
		self.default = Some (default);
		self
	}

	pub fn build (self) -> Grid <Value> {
		if self.top.is_none () { panic! () }
		if self.bottom.is_none () { panic! () }
		if self.left.is_none () { panic! () }
		if self.right.is_none () { panic! () }
		if self.default.is_none () { panic! () }
		if self.bounded.is_none () { panic! () }
		Grid {
			top: self.top.unwrap (),
			bottom: self.bottom.unwrap (),
			left: self.left.unwrap (),
			right: self.right.unwrap (),
			width: self.width.unwrap (),
			height: self.height.unwrap (),
			default: self.default.clone ().unwrap (),
			bounded: self.bounded.unwrap (),
			data: iter::repeat (self.default.unwrap ())
				.take (usize::checked_mul (self.width.unwrap (), self.height.unwrap ()).unwrap ())
				.collect (),
		}
	}

}
