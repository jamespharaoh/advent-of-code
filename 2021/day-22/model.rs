use super::*;

use input::InputCube;
use input::InputStep;

#[ derive (Clone, Copy, Debug) ]
pub struct Step {
	pub state: bool,
	pub cube: Cube,
}

impl From <InputStep> for Step {
	fn from (step: InputStep) -> Self {
		Self {
			state: step.state,
			cube: step.cube.into (),
		}
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
pub struct Cube {
	pub x0: i32, pub x1: i32,
	pub y0: i32, pub y1: i32,
	pub z0: i32, pub z1: i32,
}

impl From <InputCube> for Cube {
	fn from (cube: InputCube) -> Self {
		Self {
			x0: cube.x0, x1: cube.x1 + 1,
			y0: cube.y0, y1: cube.y1 + 1,
			z0: cube.z0, z1: cube.z1 + 1,
		}
	}
}

impl Cube {

	#[ must_use ]
	pub const fn overlaps (self, other: Self) -> bool {
		self.x0 < other.x1 && other.x0 < self.x1
			&& self.y0 < other.y1 && other.y0 < self.y1
			&& self.z0 < other.z1 && other.z0 < self.z1
	}

	#[ must_use ]
	pub fn intersect (self, other: Self) -> Option <Self> {
		self.overlaps (other).then_some (Self {
			x0: cmp::max (self.x0, other.x0),
			x1: cmp::min (self.x1, other.x1),
			y0: cmp::max (self.y0, other.y0),
			y1: cmp::min (self.y1, other.y1),
			z0: cmp::max (self.z0, other.z0),
			z1: cmp::min (self.z1, other.z1),
		} )
	}

	#[ must_use ]
	pub fn volume (self) -> i64 {
		(self.x1.pan_i64 () - self.x0.pan_i64 ())
			* (self.y1.pan_i64 () - self.y0.pan_i64 ())
			* (self.z1.pan_i64 () - self.z0.pan_i64 ())
	}

	pub const ZERO: Self = Self { x0: 0, x1: 0, y0: 0, y1: 0, z0: 0, z1: 0 };

}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Pos { pub z: i32, pub y: i32, pub x: i32 }
