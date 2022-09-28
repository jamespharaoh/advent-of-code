use super::*;

pub type Axis = pos::AxisXYZ;
pub type Coord = i16;
pub type Energy = i32;
pub type Pos = pos::PosXYZ <Coord>;

pub struct Moon {
	pub pos: Pos,
	pub vel: Pos,
}

impl Moon {

	#[ must_use ]
	pub const fn new (pos: Pos) -> Self {
		Self { pos, vel: Pos::ZERO }
	}

	#[ must_use ]
	pub fn potential_energy (& self) -> Energy {
		self.pos.x.pan_i32 ().abs () + self.pos.y.pan_i32 ().abs () + self.pos.z.pan_i32 ().abs ()
	}

	#[ must_use ]
	pub fn kinetic_energy (& self) -> Energy {
		self.vel.x.pan_i32 ().abs () + self.vel.y.pan_i32 ().abs () + self.vel.z.pan_i32 ().abs ()
	}

	#[ must_use ]
	pub fn total_energy (& self) -> Energy {
		self.potential_energy () * self.kinetic_energy ()
	}

}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct MoonAxis {
	pub pos: Coord,
	pub vel: Coord,
}
