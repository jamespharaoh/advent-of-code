use super::*;

use aoc_pos::PosGeo;
use aoc_pos::PosRowCol;
use aoc_pos::PosWXYZ;
use aoc_pos::PosXY;
use aoc_pos::PosXYZ;
use aoc_pos::PosXYZW;
use aoc_pos::PosYX;

/// Trait for values to use as indices for a [`Grid`].
///
/// For example, a two dimensional grid might be indexed with a struct containing an `x` and a `y`
/// coordinate.
///
/// This trait provides methods to translate whatever coordinate system is in use to and from a
/// single `usize` value.
///
pub trait GridPos <const DIMS: usize>: Copy + Debug + Sized {

	fn to_native (self, origin: [isize; DIMS]) -> Option <[usize; DIMS]>;
	fn to_native_offset (self) -> Option <[isize; DIMS]>;

	#[ inline ]
	#[ must_use ]
	fn to_scalar (self, origin: [isize; DIMS], size: [usize; DIMS]) -> Option <usize> {
		self.to_native (origin) ?
			.iter ().copied ()
			.zip (size.iter ().copied ())
			.fold (Ok (0), |scalar, (val, size)| {
				if size <= val { return Err (Overflow) }
				Int::add_2 (Int::mul_2 (scalar ?, size) ?, val)
			})
			.ok ()
	}

	#[ inline ]
	#[ must_use ]
	fn to_scalar_offset (self, size: [usize; DIMS]) -> Option <isize> {
		self.to_native_offset () ?
			.iter ().copied ()
			.zip (size.iter ().copied ())
			.try_fold (0, |scalar, (val, size)| {
				if size <= val.unsigned_abs () { return Err (Overflow) }
				Int::add_2 (Int::mul_2 (scalar, size.as_isize ()) ?, val)
			})
			.ok ()
	}

	fn from_native (array: [usize; DIMS], origin: [isize; DIMS]) -> Option <Self>;

	#[ inline ]
	#[ must_use ]
	fn from_scalar (
		mut scalar: usize,
		origin: [isize; DIMS],
		size: [usize; DIMS],
	) -> Option <Self> {
		let mut array = [0; DIMS];
		for idx in (0 .. DIMS).rev () {
			array [idx] = scalar % size [idx];
			scalar /= size [idx];
		}
		if scalar != 0 { return None }
		Self::from_native (array, origin)
	}

	fn size_from_native (array: [usize; DIMS]) -> Option <Self>;

}

impl <Val: Int, const DIMS: usize> GridPos <DIMS> for [Val; DIMS] {

	#[ inline ]
	fn to_native (self, origin: [isize; DIMS]) -> Option <[usize; DIMS]> {
		let mut result = [0; DIMS];
		for idx in 0 .. DIMS {
			let val_isize = self [idx].to_isize ().ok () ?;
			let adj_isize = isize::add_2 (val_isize, origin [idx]).ok () ?;
			result [idx] = adj_isize.to_usize ().ok () ?;
		}
		Some (result)
	}

	#[ inline ]
	fn to_native_offset (self) -> Option <[isize; DIMS]> {
		let mut result = [0; DIMS];
		for idx in 0 .. DIMS {
			result [idx] = self [idx].to_isize ().ok () ?;
		}
		Some (result)
	}

	#[ inline ]
	fn from_native (array: [usize; DIMS], origin: [isize; DIMS]) -> Option <Self> {
		let mut result = [Val::ZERO; DIMS];
		for idx in 0 .. DIMS {
			let val_isize = array [idx].to_isize ().ok () ?;
			let adj_isize = isize::sub_2 (val_isize, origin [idx]).ok () ?;
			result [idx] = Val::from_isize (adj_isize).ok () ?;
		}
		Some (result)
	}

	#[ inline ]
	fn size_from_native (size: [usize; DIMS]) -> Option <Self> {
		let mut result = [Val::ZERO; DIMS];
		for idx in 0 .. DIMS {
			result [idx] = Val::from_usize (size [idx]).ok () ?;
		}
		Some (result)
	}

}

impl <Val: Int> GridPos <2> for PosXY <Val> {

	#[ inline ]
	fn to_native (self, origin: [isize; 2]) -> Option <[usize; 2]> {
		GridPos::to_native ([ self.x, self.y ], origin)
	}

	#[ inline ]
	fn to_native_offset (self) -> Option <[isize; 2]> {
		GridPos::to_native_offset ([ self.x, self.y ])
	}

	#[ inline ]
	fn from_native (array: [usize; 2], origin: [isize; 2]) -> Option <Self> {
		let array = <[Val; 2]>::from_native (array, origin) ?;
		Some (Self { x: array [0], y: array [1] })
	}

	#[ inline ]
	fn size_from_native (size: [usize; 2]) -> Option <Self> {
		Some (Self {
			x: Val::from_usize (size [0]).ok () ?,
			y: Val::from_usize (size [1]).ok () ?,
		})
	}

}

impl <Val: Int> GridPos <2> for PosYX <Val> {

	#[ inline ]
	fn to_native (self, origin: [isize; 2]) -> Option <[usize; 2]> {
		GridPos::to_native ([ self.y, self.x ], origin)
	}

	#[ inline ]
	fn to_native_offset (self) -> Option <[isize; 2]> {
		GridPos::to_native_offset ([ self.y, self.x ])
	}

	#[ inline ]
	fn from_native (array: [usize; 2], origin: [isize; 2]) -> Option <Self> {
		let array = <[Val; 2]>::from_native (array, origin) ?;
		Some (Self { y: array [0], x: array [1] })
	}

	#[ inline ]
	fn size_from_native (size: [usize; 2]) -> Option <Self> {
		Some (Self {
			y: Val::from_usize (size [0]).ok () ?,
			x: Val::from_usize (size [1]).ok () ?,
		})
	}

}

impl <Val: Int> GridPos <2> for PosRowCol <Val> {

	#[ inline ]
	fn to_native (self, origin: [isize; 2]) -> Option <[usize; 2]> {
		GridPos::to_native ([ self.row, self.col ], origin)
	}

	#[ inline ]
	fn to_native_offset (self) -> Option <[isize; 2]> {
		GridPos::to_native_offset ([ self.row, self.col ])
	}

	#[ inline ]
	fn from_native (array: [usize; 2], origin: [isize; 2]) -> Option <Self> {
		let array = <[Val; 2]>::from_native (array, origin) ?;
		Some (Self { row: array [0], col: array [1] })
	}

	#[ inline ]
	fn size_from_native (size: [usize; 2]) -> Option <Self> {
		Some (Self {
			row: Val::from_usize (size [0]).ok () ?,
			col: Val::from_usize (size [1]).ok () ?,
		})
	}

}

impl <Val: Int> GridPos <2> for PosGeo <Val> {

	#[ inline ]
	fn to_native (self, origin: [isize; 2]) -> Option <[usize; 2]> {
		GridPos::to_native ([ self.n, self.e ], origin)
	}

	#[ inline ]
	fn to_native_offset (self) -> Option <[isize; 2]> {
		GridPos::to_native_offset ([ self.n, self.e ])
	}

	#[ inline ]
	fn from_native (array: [usize; 2], origin: [isize; 2]) -> Option <Self> {
		let array = <[Val; 2]>::from_native (array, origin) ?;
		Some (Self { n: array [0], e: array [1] })
	}

	#[ inline ]
	fn size_from_native (size: [usize; 2]) -> Option <Self> {
		Some (Self {
			n: Val::from_usize (size [0]).ok () ?,
			e: Val::from_usize (size [1]).ok () ?,
		})
	}

}

impl <Val: Int> GridPos <3> for PosXYZ <Val> {

	#[ inline ]
	fn to_native (self, origin: [isize; 3]) -> Option <[usize; 3]> {
		GridPos::to_native ([ self.x, self.y, self.z ], origin)
	}

	#[ inline ]
	fn to_native_offset (self) -> Option <[isize; 3]> {
		GridPos::to_native_offset ([ self.x, self.y, self.z ])
	}

	#[ inline ]
	fn from_native (array: [usize; 3], origin: [isize; 3]) -> Option <Self> {
		let array = <[Val; 3]>::from_native (array, origin) ?;
		Some (Self { x: array [0], y: array [1], z: array [2] })
	}

	#[ inline ]
	fn size_from_native (size: [usize; 3]) -> Option <Self> {
		Some (Self {
			x: Val::from_usize (size [0]).ok () ?,
			y: Val::from_usize (size [1]).ok () ?,
			z: Val::from_usize (size [2]).ok () ?,
		})
	}

}

impl <Val: Int> GridPos <4> for PosWXYZ <Val> {

	#[ inline ]
	fn to_native (self, origin: [isize; 4]) -> Option <[usize; 4]> {
		GridPos::to_native ([ self.w, self.x, self.y, self.z ], origin)
	}

	#[ inline ]
	fn to_native_offset (self) -> Option <[isize; 4]> {
		GridPos::to_native_offset ([ self.w, self.x, self.y, self.z ])
	}

	#[ inline ]
	fn from_native (array: [usize; 4], origin: [isize; 4]) -> Option <Self> {
		let array = <[Val; 4]>::from_native (array, origin) ?;
		Some (Self { w: array [0], x: array [1], y: array [2], z: array [3] })
	}

	#[ inline ]
	fn size_from_native (size: [usize; 4]) -> Option <Self> {
		Some (Self {
			w: Val::from_usize (size [0]).ok () ?,
			x: Val::from_usize (size [1]).ok () ?,
			y: Val::from_usize (size [2]).ok () ?,
			z: Val::from_usize (size [3]).ok () ?,
		})
	}

}

impl <Val: Int> GridPos <4> for PosXYZW <Val> {

	#[ inline ]
	fn to_native (self, origin: [isize; 4]) -> Option <[usize; 4]> {
		GridPos::to_native ([ self.x, self.y, self.z, self.w ], origin)
	}

	#[ inline ]
	fn to_native_offset (self) -> Option <[isize; 4]> {
		GridPos::to_native_offset ([ self.x, self.y, self.z, self.w ])
	}

	#[ inline ]
	fn from_native (array: [usize; 4], origin: [isize; 4]) -> Option <Self> {
		let array = <[Val; 4]>::from_native (array, origin) ?;
		Some (Self { x: array [0], y: array [1], z: array [2], w: array [3] })
	}

	#[ inline ]
	fn size_from_native (size: [usize; 4]) -> Option <Self> {
		Some (Self {
			x: Val::from_usize (size [0]).ok () ?,
			y: Val::from_usize (size [1]).ok () ?,
			z: Val::from_usize (size [2]).ok () ?,
			w: Val::from_usize (size [3]).ok () ?,
		})
	}

}

pub enum GridPosDisplayOrder {
	RightDown,
	RightUp,
	UpRight,
}

pub trait GridPosDisplay: GridPos <2> {
	const ORDER: GridPosDisplayOrder;
}

impl <Val: Int> GridPosDisplay for PosGeo <Val> {
	const ORDER: GridPosDisplayOrder = GridPosDisplayOrder::RightUp;
}

impl <Val: Int> GridPosDisplay for PosXY <Val> {
	const ORDER: GridPosDisplayOrder = GridPosDisplayOrder::UpRight;
}

impl <Val: Int> GridPosDisplay for PosYX <Val> {
	const ORDER: GridPosDisplayOrder = GridPosDisplayOrder::RightDown;
}
