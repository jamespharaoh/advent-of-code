use super::*;

pub type Coord = i16;
pub type Grid = GridBuf <Vec <Space>, Pos, 2>;
pub type Pos = aoc_pos::PosYX <Coord>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Space {
		#[ default ]
		Empty = [ "." ],
		Asteroid = [ "#" ],
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct DirVec {
	pub dir: Dir,
	pub mul: Coord,
}

impl DirVec {
	#[ must_use ]
	pub fn new (offset: Pos) -> Self {
		assert! (offset != Pos::ZERO);
		if offset.x == 0 {
			let sign = offset.y.signum ();
			return Self { dir: Dir (Pos { y: sign, x: 0 }), mul: offset.y.abs () }
		}
		if offset.y == 0 {
			let sign = offset.x.signum ();
			return Self { dir: Dir (Pos { y: 0, x: sign }), mul: offset.x.abs () }
		}
		let mul = calc_gcd (offset.y.abs (), offset.x.abs ());
		let dir = Dir (Pos { y: offset.y / mul, x: offset.x / mul });
		Self { dir, mul }
	}
	#[ must_use ]
	pub fn pos (self) -> Pos {
		self.dir.0 * self.mul
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
pub struct Dir (Pos);

impl Ord for Dir {
	fn cmp (& self, other: & Self) -> Ordering {
		let pos_0 = self.0;
		let pos_1 = other.0;
		fn get_dir (pos: Pos) -> u8 {
			match (pos.y.signum (), pos.x.signum ()) {
				(-1, 0) => 0,
				(-1, 1) => 1,
				(0, 1) => 2,
				(1, 1) => 3,
				(1, 0) => 4,
				(1, -1) => 5,
				(0, -1) => 6,
				(-1, -1) => 7,
				_ => unreachable! (),
			}
		}
		let key_0 = (get_dir (pos_0), pos_0.y * pos_1.x);
		let key_1 = (get_dir (pos_1), pos_1.y * pos_0.x);
		key_0.cmp (& key_1)
	}
}

impl PartialOrd for Dir {
	fn partial_cmp (& self, other: & Self) -> Option <Ordering> {
		Some (self.cmp (other))
	}
}

const fn calc_gcd (mut num_0: Coord, mut num_1: Coord) -> Coord {
	loop {
		let rem = num_0 % num_1;
		if rem == 0 { return num_1 }
		num_0 = num_1;
		num_1 = rem;
	}
}
