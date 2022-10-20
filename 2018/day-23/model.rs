use super::*;

pub type Coord = i32;
pub type Pos = pos::PosXYZ <Coord>;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Nanobot {
	pub pos: Pos,
	pub radius: Coord,
}

struct_parser_display! {
	Nanobot { pos: Pos { x, y, z }, radius } = [
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
			ppp: self.ppp.overlap (other.ppp) ?,
			pnn: self.pnn.overlap (other.pnn) ?,
			npn: self.npn.overlap (other.npn) ?,
			nnp: self.nnp.overlap (other.nnp) ?,
		})
	}

	pub fn reduce (self) -> NumResult <Option <Self>> {
		let Self { mut ppp, mut pnn, mut npn, mut nnp } = self;
		loop {
			let prev = Self { ppp, pnn, npn, nnp };
			ppp = some_or! (Self::reduce_one (ppp, [ pnn, npn, nnp ]) ?, return Ok (None));
			pnn = some_or! (Self::reduce_one (pnn, [ ppp, npn, nnp ]) ?, return Ok (None));
			npn = some_or! (Self::reduce_one (npn, [ pnn, ppp, nnp ]) ?, return Ok (None));
			nnp = some_or! (Self::reduce_one (nnp, [ pnn, npn, ppp ]) ?, return Ok (None));
			let (even, odd) = [ ppp, pnn, npn, nnp ].into_iter ()
				.filter (|val| val.start == val.end)
				.map (|val| val.start)
				.fold ((false, false), |(even, odd), val|
					if val.check_bit (0) { (even, true) } else { (true, odd) });
			if even && odd { return Ok (None) }
			if even || odd {
				for val in [ & mut ppp, & mut pnn, & mut npn, & mut nnp ] {
					if val.start.check_bit (0) != odd { val.start += Coord::ONE; }
					if val.end.check_bit (0) != odd { val.end -= Coord::ONE; }
				}
			}
			let next = Self { ppp, pnn, npn, nnp };
			if prev == next { return Ok (Some (next)) }
		}
	}

	fn reduce_one (main: CoordRange, others: [CoordRange; 3]) -> NumResult <Option <CoordRange>> {
		let others_start = chk! (others [0].start + others [1].start + others [2].start) ?;
		let others_end = chk! (others [0].end + others [1].end + others [2].end) ?;
		let start = cmp::max (main.start, - others_end);
		let end = cmp::min (main.end, - others_start);
		Ok ((start <= end).then_some (CoordRange { start, end }))
	}

	pub const OPEN: Self = Self {
		ppp: CoordRange::OPEN,
		pnn: CoordRange::OPEN,
		npn: CoordRange::OPEN,
		nnp: CoordRange::OPEN,
	};

}

#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct CoordRange {
	pub start: Coord,
	pub end: Coord,
}

impl CoordRange {

	pub fn new (pos: Pos, radius: Coord, sign: Pos) -> NumResult <Self> {
		let base = chk! (pos.x * sign.x + pos.y * sign.y + pos.z * sign.z) ?;
		Ok (Self {
			start: chk! (base - radius) ?,
			end: chk! (base + radius) ?,
		})
	}

	#[ must_use ]
	pub fn overlap (self, other: Self) -> Option <Self> {
		(self.start <= other.end && other.start <= self.end).then (|| Self {
			start: cmp::max (self.start, other.start),
			end: cmp::min (self.end, other.end),
		})
	}

	#[ must_use ]
	pub fn contains (& self, val: Coord) -> bool {
		(self.start ..= self.end).contains (& val)
	}

	#[ must_use ]
	pub fn dist (& self) -> Coord {
		if Coord::ZERO < self.start { return self.start }
		if self.end < Coord::ZERO { return - self.end }
		Coord::ZERO
	}

	pub const OPEN: Self = Self { start: Coord::MIN, end: Coord::MAX };

}

impl Debug for CoordRange {
	fn fmt (& self, fmtr: & mut fmt::Formatter) -> fmt::Result {
		let & Self { start, end } = self;
		if self.start == self.end {
			write! (fmtr, "{start}")
		} else {
			write! (fmtr, "{start} ..= {end}")
		}
	}
}

pub mod signs {
	use super::*;
	pub const PPP: Pos = Pos { x: Coord::ONE, y: Coord::ONE, z: Coord::ONE };
	pub const PNN: Pos = Pos { x: Coord::ONE, y: Coord::NEG_ONE, z: Coord::NEG_ONE };
	pub const NPN: Pos = Pos { x: Coord::NEG_ONE, y: Coord::ONE, z: Coord::NEG_ONE };
	pub const NNP: Pos = Pos { x: Coord::NEG_ONE, y: Coord::NEG_ONE, z: Coord::ONE };
}
