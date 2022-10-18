use super::*;

use input::Input;

pub type Coord = u16;
pub type Id = u32;
pub type Pos = pos::PosYX <Coord>;

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub struct Claim {
	pub id: Id,
	pub square: Square,
}

impl Claim {
	pub fn build_vec (input: & Input) -> GenResult <Vec <Self>> {
		let mut result = Vec::new ();
		let mut ids = HashSet::new ();
		for claim in & input.claims {
			if ! ids.insert (claim.id) {
				return Err (format! ("Duplicated claim id: {}", claim.id).into ());
			}
			result.push (Self {
				id: claim.id,
				square: Square::new_size (claim.left, claim.top, claim.width, claim.height) ?,
			});
		}
		Ok (result)
	}
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub struct Square {
	left: Coord,
	top: Coord,
	right: Coord,
	bottom: Coord,
}

impl Square {

	#[ inline ]
	pub fn new (left: Coord, top: Coord, right: Coord, bottom: Coord) -> GenResult <Self> {
		if right <= left || bottom <= top {
			return Err (format! ("Invalid square: left={left} top={top} right={right} \
				bottom={bottom}").into ());
		}
		Ok (Self { left, top, right, bottom })
	}

	#[ inline ]
	pub fn new_size (left: Coord, top: Coord, width: Coord, height: Coord) -> GenResult <Self> {
		if width == 0 || height == 0 {
			return Err (format! ("Invalid square: left={left} top={top} width={width} \
				height={height}").into ());
		}
		let right = chk! (left + width) ?;
		let bottom = chk! (top + height) ?;
		Ok (Self { left, top, right, bottom })
	}

	#[ inline ]
	#[ must_use ]
	pub const fn left (self) -> Coord { self.left }

	#[ inline ]
	#[ must_use ]
	pub const fn top (self) -> Coord { self.top }

	#[ inline ]
	#[ must_use ]
	pub const fn right (self) -> Coord { self.right }

	#[ inline ]
	#[ must_use ]
	pub const fn bottom (self) -> Coord { self.bottom }

	#[ inline ]
	#[ must_use ]
	pub const fn width (self) -> Coord { self.right - self.left }

	#[ inline ]
	#[ must_use ]
	pub const fn height (self) -> Coord { self.bottom - self.top }

	#[ inline ]
	#[ must_use ]
	pub fn area (self) -> u32 { self.width ().pan_u32 () * self.height ().pan_u32 () }

	#[ inline ]
	#[ must_use ]
	pub fn overlap (self, other: Self) -> Option <Self> {
		let left = cmp::max (self.left, other.left);
		let right = cmp::min (self.right, other.right);
		if right <= left { return None }
		let top = cmp::max (self.top, other.top);
		let bottom = cmp::min (self.bottom, other.bottom);
		if bottom <= top { return None }
		Some (Self { left, top, right, bottom })
	}

	#[ inline ]
	#[ must_use ]
	pub fn remove (self, other: Self) -> ArrayVec <Self, 4> {
		if Self::overlap (self, other).is_none () {
			return array_vec! [ self ];
		}
		let mut result = ArrayVec::new ();
		if self.left < other.left {
			result.push (Self {
				left: self.left,
				top: self.top,
				right: other.left,
				bottom: self.bottom,
			});
		}
		if self.top < other.top {
			result.push (Self {
				left: cmp::max (self.left, other.left),
				top: self.top,
				right: cmp::min (self.right, other.right),
				bottom: other.top,
			});
		}
		if other.bottom < self.bottom {
			result.push (Self {
				left: cmp::max (self.left, other.left),
				top: other.bottom,
				right: cmp::min (self.right, other.right),
				bottom: self.bottom,
			});
		}
		if other.right < self.right {
			result.push (Self {
				left: other.right,
				top: self.top,
				right: self.right,
				bottom: self.bottom,
			});
		}
		result
	}

	#[ inline ]
	#[ must_use ]
	pub fn bound (self, other: Self) -> Self {
		Self {
			left: cmp::min (self.left, other.left),
			right: cmp::max (self.right, other.right),
			top: cmp::min (self.top, other.top),
			bottom: cmp::max (self.bottom, other.bottom),
		}
	}

}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn square_remove () {
		assert_eq! (
			array_vec! [
				Square { left: 15, top: 10, right: 20, bottom: 15 } ],
			Square::remove (
				Square { left: 10, top: 10, right: 20, bottom: 15 },
				Square { left: 5, top: 5, right: 15, bottom: 20 }));
		assert_eq! (
			array_vec! [
				Square { left: 10, top: 5, right: 15, bottom: 10 },
				Square { left: 10, top: 15, right: 15, bottom: 20 } ],
			Square::remove (
				Square { left: 10, top: 5, right: 15, bottom: 20 },
				Square { left: 5, top: 10, right: 20, bottom: 15 }));
		assert_eq! (
			array_vec! [
				Square { left: 5, top: 10, right: 10, bottom: 15 },
				Square { left: 15, top: 10, right: 20, bottom: 15 } ],
			Square::remove (
				Square { left: 5, top: 10, right: 20, bottom: 15 },
				Square { left: 10, top: 5, right: 15, bottom: 20 }));
		assert_eq! (
			array_vec! [
				Square { left: 5, top: 10, right: 10, bottom: 20 },
				Square { left: 10, top: 15, right: 15, bottom: 20 } ],
			Square::remove (
				Square { left: 5, top: 10, right: 15, bottom: 20 },
				Square { left: 10, top: 5, right: 20, bottom: 15 }));
		assert_eq! (
			array_vec! [
				Square { left: 5, top: 5, right: 10, bottom: 20 },
				Square { left: 10, top: 5, right: 15, bottom: 10 },
				Square { left: 10, top: 15, right: 15, bottom: 20 },
				Square { left: 15, top: 5, right: 20, bottom: 20 } ],
			Square::remove (
				Square { left: 5, top: 5, right: 20, bottom: 20 },
				Square { left: 10, top: 10, right: 15, bottom: 15 }));
		assert_eq! (
			array_vec! [
				Square { left: 10, top: 5, right: 15, bottom: 10 },
				Square { left: 15, top: 5, right: 20, bottom: 15 } ],
			Square::remove (
				Square { left: 10, top: 5, right: 20, bottom: 15 },
				Square { left: 5, top: 10, right: 15, bottom: 20 }));
		assert_eq! (
			array_vec! [
				Square { left: 10, top: 15, right: 15, bottom: 20 } ],
			Square::remove (
				Square { left: 10, top: 10, right: 15, bottom: 20 },
				Square { left: 5, top: 5, right: 20, bottom: 15 }));
		assert_eq! (
			array_vec! [
				Square { left: 5, top: 10, right: 10, bottom: 20 },
				Square { left: 10, top: 15, right: 15, bottom: 20 },
				Square { left: 15, top: 10, right: 20, bottom: 20 } ],
			Square::remove (
				Square { left: 5, top: 10, right: 20, bottom: 20 },
				Square { left: 10, top: 5, right: 15, bottom: 15 }));
	}

}
