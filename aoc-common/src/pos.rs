use super::*;
use nums::Int;

#[ derive (Clone, Copy, fmt::Debug, Eq, Hash, PartialEq) ]
pub struct PosXY <Val> { pub x: Val, pub y: Val }

impl <Val> PosXY <Val> {
	pub fn adjacent_4 (& self) -> ArrayVec <PosXY <Val>, 4> where Val: Int {
		let mut result = ArrayVec::new ();
		let (x, y) = (self.x, self.y);
		if self.x > Val::MIN { result.push (PosXY { x: x - Val::ONE, y }); }
		if self.x < Val::MAX { result.push (PosXY { x: x + Val::ONE, y }); }
		if self.y > Val::MIN { result.push (PosXY { x, y: y - Val::ONE }); }
		if self.y < Val::MAX { result.push (PosXY { x, y: y + Val::ONE }); }
		result
	}
}

#[ derive (Clone, Copy, fmt::Debug, Eq, Hash, PartialEq) ]
pub struct PosYX <Val> { pub y: Val, pub x: Val }

impl <Val> PosYX <Val> {
	pub fn adjacent_4 (& self) -> ArrayVec <PosYX <Val>, 4> where Val: Int {
		let mut result = ArrayVec::new ();
		let PosYX { y, x } = * self;
		if self.y > Val::MIN { result.push (PosYX { x, y: y - Val::ONE }); }
		if self.y < Val::MAX { result.push (PosYX { x, y: y + Val::ONE }); }
		if self.x > Val::MIN { result.push (PosYX { x: x - Val::ONE, y }); }
		if self.x < Val::MAX { result.push (PosYX { x: x + Val::ONE, y }); }
		result
	}
}

impl <Val: Int> PosYX <Val> {
	pub const ZERO: Self = Self::zero ();
	pub const fn zero () -> Self { Self { y: Val::ZERO, x: Val::ZERO } }
}

impl <Val: Int> Add for PosYX <Val> {
	type Output = Self;
	fn add (self, other: Self) -> Self { PosYX { y: self.y + other.y, x: self.x + other.x } }
}

#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct PosXYZ <Val> { pub x: Val, pub y: Val, pub z: Val }

impl <Val: Int> PosXYZ <Val> {
	pub fn zero () -> Self { PosXYZ { x: Val::ZERO, y: Val::ZERO, z: Val::ZERO } }
	pub fn max () -> Self { PosXYZ { x: Val::MAX, y: Val::MAX, z: Val::MAX } }
	pub fn abs_diff (self, other: Self) -> Self {
		if self < other { other - self } else { self - other }
	}
}

impl <Val: Int> fmt::Debug for PosXYZ <Val> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "PosXYZ ({:?}, {:?}, {:?})", self.x, self.y, self.z) ?;
		Ok (())
	}
}

impl <Val> ops::Add for PosXYZ <Val> where Val: Add <Output = Val> {
	type Output = Self;
	fn add (self, other: Self) -> Self {
		PosXYZ { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
	}
}

impl <Val> Sub for PosXYZ <Val> where Val: Sub <Output = Val> {
	type Output = Self;
	fn sub (self, other: Self) -> Self {
		PosXYZ { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
	}
}

impl <Val> Neg for PosXYZ <Val> where Val: Neg <Output = Val> {
	type Output = Self;
	fn neg (self) -> Self {
		PosXYZ { x: - self.x, y: - self.y, z: - self.z }
	}
}
