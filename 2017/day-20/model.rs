use super::*;

pub use particle::Particle;

pub type Axis = pos::AxisXYZ;
pub type Coord = i16;
pub type CoordAbs = u16;
pub type Pos = pos::PosXYZ <Coord>;

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum Aspect { Pos, Vel, Acc }

mod particle {

	use super::*;

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub struct Particle {
		pub pos: Pos,
		pub vel: Pos,
		pub acc: Pos,
	}

	impl Display for Particle {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter,
				"p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>",
				self.pos.x, self.pos.y, self.pos.z,
				self.vel.x, self.vel.y, self.vel.z,
				self.acc.x, self.acc.y, self.acc.z) ?;
			Ok (())
		}
	}

	impl <'inp> FromParser <'inp> for Particle {
		fn from_parser (parser: & mut Parser) -> ParseResult <Self> {
			parse! (parser,
				"p=<", pos_x, ",", pos_y, ",", pos_z, ">, ",
				"v=<", vel_x, ",", vel_y, ",", vel_z, ">, ",
				"a=<", acc_x, ",", acc_y, ",", acc_z, ">");
			let pos = Pos { x: pos_x, y: pos_y, z: pos_z };
			let vel = Pos { x: vel_x, y: vel_y, z: vel_z };
			let acc = Pos { x: acc_x, y: acc_y, z: acc_z };
			Ok (Self { pos, vel, acc })
		}
	}

	impl Index <Aspect> for Particle {
		type Output = Pos;
		fn index (& self, aspect: Aspect) -> & Pos {
			match aspect {
				Aspect::Pos => & self.pos,
				Aspect::Vel => & self.vel,
				Aspect::Acc => & self.acc,
			}
		}
	}

}
