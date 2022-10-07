use super::*;

use aoc_pos::PosGeo;
use aoc_pos::PosGeoHexLat;
use aoc_pos::PosRowCol;
use aoc_pos::PosWXYZ;
use aoc_pos::PosXY;
use aoc_pos::PosXYZ;
use aoc_pos::PosXYZW;
use aoc_pos::PosYX;

/// Trait for values to use as indices for a [`GridView`].
///
/// For example, a two dimensional grid might be indexed with a struct containing an `x` and a `y`
/// coordinate.
///
/// This trait provides methods to translate whatever coordinate system is in use to and from a
/// single `usize` value.
///
pub trait GridPos <const DIMS: usize>: Copy + Debug + Default + Eq + Sized {

	type Coord: Int;

	fn to_array (self) -> [Self::Coord; DIMS];
	fn from_array (array: [Self::Coord; DIMS]) -> Self;

	#[ inline ]
	fn map <MapFn, Output> (self, map_fn: MapFn) -> [Output; DIMS]
			where MapFn: FnMut (Self::Coord) -> Output {
		self.to_array ().map (map_fn)
	}

	#[ inline ]
	fn to_native (self, start: Self) -> Option <Self> {
		let self_arr = self.to_array ();
		let start_arr = start.to_array ();
		let mut result_arr = [Self::Coord::ZERO; DIMS];
		for idx in 0 .. DIMS {
			let result = chk! (self_arr [idx] - start_arr [idx]).ok () ?;
			if result < Self::Coord::ZERO { return None }
			result_arr [idx] = result;
		}
		Some (Self::from_array (result_arr))
	}

	#[ inline ]
	#[ must_use ]
	fn native_to_index (self, size: Self) -> Option <isize> {
		self.to_array ().iter ()
			.zip (size.to_array ().iter ())
			.map (|(& val, & size)| (val.qck_isize (), size.qck_isize ()))
			.try_fold (0, |idx, (val, size)| {
				if size <= val { return Err (Overflow) }
				chk! (idx * size + val)
			})
			.ok ()
	}

	#[ inline ]
	fn from_native (native: Self, start: Self) -> Option <Self> {
		let native_arr = native.to_array ();
		let start_arr = start.to_array ();
		let mut result_arr = [Self::Coord::ZERO; DIMS];
		for idx in 0 .. DIMS {
			result_arr [idx] = chk! (native_arr [idx] + start_arr [idx]).ok () ?;
		}
		Some (Self::from_array (result_arr))
	}

}

impl <Val: Int> GridPos <2> for PosXY <Val> {

	type Coord = Val;

	#[ inline ]
	fn to_array (self) -> [Val; 2] {
		[ self.x, self.y ]
	}

	#[ inline ]
	fn from_array (array: [Val; 2]) -> Self {
		Self { x: array [0], y: array [1] }
	}

}

impl <Val: Int> GridPos <2> for PosYX <Val> {

	type Coord = Val;

	#[ inline ]
	fn to_array (self) -> [Val; 2] {
		[ self.y, self.x ]
	}

	#[ inline ]
	fn from_array (array: [Val; 2]) -> Self {
		Self { y: array [0], x: array [1] }
	}

}

impl <Val: Int> GridPos <2> for PosRowCol <Val> {

	type Coord = Val;

	#[ inline ]
	fn to_array (self) -> [Val; 2] {
		[ self.row, self.col ]
	}

	#[ inline ]
	fn from_array (array: [Val; 2]) -> Self {
		Self { row: array [0], col: array [1] }
	}

}

impl <Val: Int> GridPos <2> for PosGeo <Val> {

	type Coord = Val;

	#[ inline ]
	fn to_array (self) -> [Val; 2] {
		[ self.n, self.e ]
	}

	#[ inline ]
	fn from_array (array: [Val; 2]) -> Self {
		Self { n: array [0], e: array [1] }
	}

}

impl <Val: Int> GridPos <2> for PosGeoHexLat <Val> {

	type Coord = Val;

	#[ inline ]
	fn to_array (self) -> [Val; 2] {
		[ self.nw, self.e ]
	}

	#[ inline ]
	fn from_array (array: [Val; 2]) -> Self {
		Self { nw: array [0], e: array [1] }
	}

}

impl <Val: Int> GridPos <3> for PosXYZ <Val> {

	type Coord = Val;

	#[ inline ]
	fn to_array (self) -> [Val; 3] {
		[ self.x, self.y, self.z ]
	}

	#[ inline ]
	fn from_array (array: [Val; 3]) -> Self {
		Self { x: array [0], y: array [1], z: array [2] }
	}

}

impl <Val: Int> GridPos <4> for PosWXYZ <Val> {

	type Coord = Val;

	#[ inline ]
	fn to_array (self) -> [Val; 4] {
		[ self.w, self.x, self.y, self.z ]
	}

	#[ inline ]
	fn from_array (array: [Val; 4]) -> Self {
		Self { w: array [0], x: array [1], y: array [2], z: array [3] }
	}

}

impl <Val: Int> GridPos <4> for PosXYZW <Val> {

	type Coord = Val;

	#[ inline ]
	fn to_array (self) -> [Val; 4] {
		[ self.x, self.y, self.z, self.w ]
	}

	#[ inline ]
	fn from_array (array: [Val; 4]) -> Self {
		Self { x: array [0], y: array [1], z: array [2], w: array [3] }
	}

}
