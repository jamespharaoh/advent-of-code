use super::*;
use num::PrimInt;

#[ derive (Clone, Copy, fmt::Debug, Eq, Hash, PartialEq) ]
pub struct PosXY <Val: PrimInt> { pub x: Val, pub y: Val }

impl <Val: PrimInt> PosXY <Val> {
	pub fn adjacent_4 (& self) -> ArrayVec <PosXY <Val>, 4> {
		let mut result = ArrayVec::new ();
		let (x, y) = (self.x, self.y);
		if self.x > Val::min_value () {
			result.push (PosXY { x: x - Val::one (), y });
		}
		if self.x < Val::max_value () {
			result.push (PosXY { x: x + Val::one (), y });
		}
		if self.y > Val::min_value () {
			result.push (PosXY { x, y: y - Val::one (), });
		}
		if self.y < Val::max_value () {
			result.push (PosXY { x, y: y + Val::one (), });
		}
		result
	}
}

#[ derive (Clone, Copy, fmt::Debug, Eq, Hash, PartialEq) ]
pub struct PosYX <Val: PrimInt> { pub y: Val, pub x: Val }

impl <Val: PrimInt> PosYX <Val> {
	pub fn adjacent_4 (& self) -> ArrayVec <PosYX <Val>, 4> {
		let mut result = ArrayVec::new ();
		let PosYX { y, x } = * self;
		if self.y > Val::min_value () {
			result.push (PosYX { x, y: y - Val::one (), });
		}
		if self.y < Val::max_value () {
			result.push (PosYX { x, y: y + Val::one (), });
		}
		if self.x > Val::min_value () {
			result.push (PosYX { x: x - Val::one (), y });
		}
		if self.x < Val::max_value () {
			result.push (PosYX { x: x + Val::one (), y });
		}
		result
	}
}

impl <Val: PrimInt> PosYX <Val> {
	pub fn zero () -> Self { Self { y: Val::zero (), x: Val::zero () } }
}

impl <Val: PrimInt> ops::Add for PosYX <Val> {
	type Output = Self;
	fn add (self, other: Self) -> Self { PosYX { y: self.y + other.y, x: self.x + other.x } }
}

#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct PosXYZ <Val: PrimInt> { pub x: Val, pub y: Val, pub z: Val }

impl <Val: PrimInt> PosXYZ <Val> {
	pub fn zero () -> Self { PosXYZ { x: Val::zero (), y: Val::zero (), z: Val::zero () } }
	pub fn max () -> Self { PosXYZ { x: Val::max_value (), y: Val::max_value (), z: Val::max_value () } }
	pub fn abs_diff (self, other: Self) -> Self {
		if self < other { other - self } else { self - other }
	}
}

impl <Val: PrimInt + fmt::Debug> fmt::Debug for PosXYZ <Val> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "PosXYZ ({:?}, {:?}, {:?})", self.x, self.y, self.z) ?;
		Ok (())
	}
}

impl <Val: PrimInt> ops::Add for PosXYZ <Val> {
	type Output = Self;
	fn add (self, other: Self) -> Self {
		PosXYZ { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
	}
}

impl <Val: PrimInt> ops::Sub for PosXYZ <Val> {
	type Output = Self;
	fn sub (self, other: Self) -> Self {
		PosXYZ { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
	}
}
