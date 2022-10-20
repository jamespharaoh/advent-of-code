use super::*;

pub type Axis = pos::AxisXYZ;
pub type Coord = i32;
pub type CoordAbs = u32;
pub type Pos = pos::PosXYZ <Coord>;

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum Aspect { Pos, Vel, Acc }

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub struct Particle {
	pub pos: Pos,
	pub vel: Pos,
	pub acc: Pos,
}

struct_parser_display! {
	Particle {
		pos: Pos { x: pos_x, y: pos_y, z: pos_z },
		vel: Pos { x: vel_x, y: vel_y, z: vel_z },
		acc: Pos { x: acc_x, y: acc_y, z: acc_z },
	} = [
		"p=<", pos_x, ",", pos_y, ",", pos_z, ">, ",
		"v=<", vel_x, ",", vel_y, ",", vel_z, ">, ",
		"a=<", acc_x, ",", acc_y, ",", acc_z, ">",
	]
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
