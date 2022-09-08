use super::*;

pub type Coord = i32;
pub type Pos = pos::PosXYZ <Coord>;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Nanobot {
	pub pos: Pos,
	pub radius: Coord,
}

struct_parser_display! {
	Nanobot {
		pos: Pos { x, y, z },
		radius,
	} = [
		"pos=<", x, ",", y, ",", z, ">, ",
		"r=", radius,
	]
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Region {
	pub ppp: CoordRange,
	pub pnn: CoordRange,
	pub npn: CoordRange,
	pub nnp: CoordRange,
}

impl Region {

	pub fn from_bot (bot: & Nanobot) -> NumResult <Self> {
		Ok (Self {
			ppp: CoordRange::new (bot.pos, bot.radius, signs::PPP) ?,
			pnn: CoordRange::new (bot.pos, bot.radius, signs::PNN) ?,
			npn: CoordRange::new (bot.pos, bot.radius, signs::NPN) ?,
			nnp: CoordRange::new (bot.pos, bot.radius, signs::NNP) ?,
		})
	}

	#[ must_use ]
	pub fn overlap (self, other: Self) -> Option <Self> {
		Some (Self {
			ppp: some_or! (self.ppp.overlap (other.ppp), return None),
			pnn: some_or! (self.pnn.overlap (other.pnn), return None),
			npn: some_or! (self.npn.overlap (other.npn), return None),
			nnp: some_or! (self.nnp.overlap (other.nnp), return None),
		})
	}

	#[ must_use ]
	pub fn dist (self) -> Coord {
		[
			self.ppp.dist (),
			self.pnn.dist (),
			self.npn.dist (),
			self.nnp.dist (),
		].iter ().copied ().max ().unwrap ()
	}

	pub const OPEN: Self = Self {
		ppp: CoordRange::OPEN,
		pnn: CoordRange::OPEN,
		npn: CoordRange::OPEN,
		nnp: CoordRange::OPEN,
	};

}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct CoordRange {
	pub start: Coord,
	pub end: Coord,
}

impl CoordRange {

	pub fn new (pos: Pos, radius: Coord, sign: Pos) -> NumResult <Self> {
		let base = Coord::add_3 (
			Coord::mul_2 (pos.x, sign.x) ?,
			Coord::mul_2 (pos.y, sign.y) ?,
			Coord::mul_2 (pos.z, sign.z) ?,
		) ?;
		Ok (Self {
			start: Coord::sub_2 (base, radius) ?,
			end: Coord::add_3 (base, radius, Coord::ONE) ?,
		})
	}

	#[ must_use ]
	pub fn overlap (self, other: Self) -> Option <Self> {
		(self.start < other.end && other.start < self.end).then (|| Self {
			start: cmp::max (self.start, other.start),
			end: cmp::min (self.end, other.end),
		})
	}

	#[ must_use ]
	pub const fn dist (self) -> Coord {
		if self.start > Coord::ZERO {
			self.start
		} else if self.end <= Coord::ZERO {
			Coord::ONE - self.end
		} else {
			Coord::ZERO
		}
	}

	pub const OPEN: Self = Self { start: Coord::MIN, end: Coord::MAX };

}

pub mod signs {
	use super::*;
	pub const PPP: Pos = Pos { x: Coord::ONE, y: Coord::ONE, z: Coord::ONE };
	pub const PNN: Pos = Pos { x: Coord::ONE, y: Coord::NEG_ONE, z: Coord::NEG_ONE };
	pub const NPN: Pos = Pos { x: Coord::NEG_ONE, y: Coord::ONE, z: Coord::NEG_ONE };
	pub const NNP: Pos = Pos { x: Coord::NEG_ONE, y: Coord::NEG_ONE, z: Coord::ONE };
}
