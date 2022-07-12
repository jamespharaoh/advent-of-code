use super::*;

#[ derive (Clone, Copy, fmt::Debug, Eq, Hash, PartialEq) ]
pub struct PosXY <Val: num::PrimInt> { pub x: Val, pub y: Val }

impl <Val: num::PrimInt> PosXY <Val> {
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
pub struct PosYX <Val: num::PrimInt> { pub y: Val, pub x: Val }

impl <Val: num::PrimInt> PosYX <Val> {
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

impl <Val: num::PrimInt> PosYX <Val> {
	pub fn zero () -> Self { Self { y: Val::zero (), x: Val::zero () } }
}

impl <Val: num::PrimInt> ops::Add for PosYX <Val> {
	type Output = Self;
	fn add (self, other: Self) -> Self { PosYX { y: self.y + other.y, x: self.x + other.x } }
}
