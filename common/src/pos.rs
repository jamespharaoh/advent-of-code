use super::*;
use nums::Int;

pub use coord::Coord;
pub use dim_2::PosXY;
pub use dim_2::PosYX;
pub use dim_2::PosGeo;
pub use dim_2::PosRowCol;
pub use dim_3::PosXYZ;

macro_rules! pos_ops {

	( $name:ident : Debug $(, $rest:tt)* ) => {
		impl <Val: Int> Debug for $name <Val> {
			#[ inline ]
			fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
				let self_coords = self.coord_to_array ();
				formatter.write_str ("PosXYZ (") ?;
				for idx in 0 .. self_coords.len () {
					if idx != 0 { formatter.write_str (", ") ?; }
					Debug::fmt (& self_coords [idx], formatter) ?;
				}
				formatter.write_str (")") ?;
				Ok (())
			}
		}
		pos_ops! ($name: $($rest),*);
	};

	( $name:ident : Add $(, $rest:tt)* ) => {
		impl <Val: Int> Add <$name <Val::Signed>> for $name <Val> {
			type Output = Self;
			#[ inline ]
			fn add (self, other: $name <Val::Signed>) -> Self {
				let self_coords = self.coord_to_array ();
				let other_coords = other.coord_to_array ();
				let mut result_coords = Self::ZERO.coord_to_array ();
				for idx in 0 .. self_coords.len () {
					result_coords [idx] =
						self_coords [idx].add_signed (other_coords [idx]).unwrap ();
				}
				Self::coord_from_array (result_coords)
			}
		}
		pos_ops! ($name: $($rest),*);
	};
	( $name:ident : Neg $(, $rest:tt)* ) => {
		impl <Val: Int + Neg <Output = Val>> Neg for $name <Val> {
			type Output = Self;
			#[ inline ]
			fn neg (self) -> Self {
				let self_coords = self.coord_to_array ();
				let mut result_coords = Self::ZERO.coord_to_array ();
				for idx in 0 .. self_coords.len () {
					result_coords [idx] = - self_coords [idx];
				}
				Self::coord_from_array (result_coords)
			}
		}
		pos_ops! ($name: $($rest),*);
	};
	( $name:ident : Rem $(, $rest:tt)* ) => {
		impl <Val: Int> Rem for $name <Val> {
			type Output = Self;
			#[ inline ]
			fn rem (self, other: $name <Val>) -> Self {
				let self_coords = self.coord_to_array ();
				let other_coords = other.coord_to_array ();
				let mut result_coords = Self::ZERO.coord_to_array ();
				for idx in 0 .. self_coords.len () {
					result_coords [idx] = self_coords [idx] % other_coords [idx];
				}
				Self::coord_from_array (result_coords)
			}
		}
		pos_ops! ($name: $($rest),*);
	};
	( $name:ident : Sub $(, $rest:tt)* ) => {
		impl <Val: Int> Sub <$name <Val::Signed>> for $name <Val> {
			type Output = Self;
			#[ inline ]
			fn sub (self, other: $name <Val::Signed>) -> Self {
				let self_coords = self.coord_to_array ();
				let other_coords = other.coord_to_array ();
				let mut result_coords = Self::ZERO.coord_to_array ();
				for idx in 0 .. self_coords.len () {
					result_coords [idx] =
						self_coords [idx].sub_signed (other_coords [idx]).unwrap ();
				}
				Self::coord_from_array (result_coords)
			}
		}
		pos_ops! ($name: $($rest),*);
	};
	( $name:ident : ) => {};

}

mod coord {

	use super::*;

	pub trait Coord <const DIMS: usize>: Copy + Debug + Sized {

		type Val: Int;
		type Signed;

		fn coord_to_array (self) -> [Self::Val; DIMS];
		fn coord_from_array (array: [Self::Val; DIMS]) -> Self;

		#[ inline ]
		#[ must_use ]
		fn zero () -> Self {
			Self::coord_from_array ([Self::Val::ZERO; DIMS])
		}

	}

}

mod dim_2 {

	use super::*;

	pub use geo::PosGeo;
	pub use row_col::PosRowCol;
	pub use xy::PosXY;
	pub use yx::PosYX;

	mod xy {

		use super::*;

		#[ derive (Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct PosXY <Val> { pub x: Val, pub y: Val }

		impl <Val: Int> PosXY <Val> {

			pub const ZERO: Self = Self { x: Int::ZERO, y: Int::ZERO };

			#[ inline ]
			pub fn adjacent_4 (& self) -> ArrayVec <Self, 4> {
				let mut result = ArrayVec::new ();
				let (x, y) = (self.x, self.y);
				if self.x > Val::MIN { result.push (Self { x: x - Val::ONE, y }); }
				if self.x < Val::MAX { result.push (Self { x: x + Val::ONE, y }); }
				if self.y > Val::MIN { result.push (Self { x, y: y - Val::ONE }); }
				if self.y < Val::MAX { result.push (Self { x, y: y + Val::ONE }); }
				result
			}

		}

		impl <Val: Int> Coord <2> for PosXY <Val> {

			type Val = Val;
			type Signed = PosXY <Val::Signed>;

			#[ inline ]
			fn coord_to_array (self) -> [Val; 2] {
				[ self.x, self.y ]
			}

			#[ inline ]
			fn coord_from_array (arr: [Val; 2]) -> Self {
				Self { x: arr [0], y: arr [1] }
			}

		}

		pos_ops! (PosXY: Debug);
		pos_ops! (PosXY: Add, Neg, Sub, Rem);

	}

	mod yx {

		use super::*;

		#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct PosYX <Val> { pub y: Val, pub x: Val }

		impl <Val: Int> PosYX <Val> {

			pub const ZERO: Self = Self { y: Val::ZERO, x: Val::ZERO };

			#[ inline ]
			pub fn adjacent_4 (& self) -> ArrayVec <Self, 4> where Val: Int {
				let mut result = ArrayVec::new ();
				let Self { y, x } = * self;
				if self.y > Val::MIN { result.push (Self { x, y: y - Val::ONE }); }
				if self.y < Val::MAX { result.push (Self { x, y: y + Val::ONE }); }
				if self.x > Val::MIN { result.push (Self { x: x - Val::ONE, y }); }
				if self.x < Val::MAX { result.push (Self { x: x + Val::ONE, y }); }
				result
			}

			#[ inline ]
			pub fn adjacent_8 (& self) -> ArrayVec <Self, 8> where Val: Int {
				let mut result = ArrayVec::new ();
				let Self { y, x } = * self;
				if self.y > Val::MIN {
					let y = y - Val::ONE;
					if self.x > Val::MIN { result.push (Self { x: x - Val::ONE, y }); }
					result.push (Self { y, x });
					if self.x < Val::MAX { result.push (Self { x: x + Val::ONE, y }); }
				}
				if self.x > Val::MIN { result.push (Self { x: x - Val::ONE, y }); }
				if self.x < Val::MAX { result.push (Self { x: x + Val::ONE, y }); }
				if self.y < Val::MAX {
					let y = y + Val::ONE;
					if self.x > Val::MIN { result.push (Self { x: x - Val::ONE, y }); }
					result.push (Self { y, x });
					if self.x < Val::MAX { result.push (Self { x: x + Val::ONE, y }); }
				}
				result
			}

		}

		impl <Val: Int> Coord <2> for PosYX <Val> {

			type Val = Val;
			type Signed = PosYX <Val::Signed>;

			#[ inline ]
			fn coord_to_array (self) -> [Val; 2] {
				[ self.x, self.y ]
			}

			#[ inline ]
			fn coord_from_array (arr: [Val; 2]) -> Self {
				Self { y: arr [0], x: arr [1] }
			}

		}

		pos_ops! (PosYX: Debug);
		pos_ops! (PosYX: Add, Neg, Rem, Sub);

	}

	mod geo {

		use super::*;

		#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct PosGeo <Val> { pub n: Val, pub e: Val }

		impl <Val: Int> PosGeo <Val> {

			pub const ZERO: Self = Self { n: Val::ZERO, e: Val::ZERO };

			#[ inline ]
			#[ must_use ]
			pub fn north (& self, num: Val) -> Self {
				Self { n: self.n.safe_add (num), e: self.e }
			}

			#[ inline ]
			#[ must_use ]
			pub fn south (& self, num: Val) -> Self {
				Self { n: self.n.safe_sub (num), e: self.e }
			}

			#[ inline ]
			#[ must_use ]
			pub fn east (& self, num: Val) -> Self {
				Self { n: self.n, e: self.e.safe_add (num) }
			}

			#[ inline ]
			#[ must_use ]
			pub fn west (& self, num: Val) -> Self {
				Self { n: self.n, e: self.e.safe_sub (num) }
			}

			#[ inline ]
			pub fn adjacent_4 (& self) -> ArrayVec <Self, 4> where Val: Int {
				let mut result = ArrayVec::new ();
				let Self { n, e } = * self;
				if n > Val::MIN { result.push (Self { n: n - Val::ONE, e }); }
				if n < Val::MAX { result.push (Self { n: n + Val::ONE, e }); }
				if e > Val::MIN { result.push (Self { n, e: e - Val::ONE }); }
				if e < Val::MAX { result.push (Self { n, e: e + Val::ONE }); }
				result
			}

		}

		impl <Val: Int> Coord <2> for PosGeo <Val> {

			type Val = Val;
			type Signed = PosGeo <Val::Signed>;

			#[ inline ]
			fn coord_to_array (self) -> [Val; 2] {
				[ self.n, self.e ]
			}

			#[ inline ]
			fn coord_from_array (arr: [Val; 2]) -> Self {
				Self { n: arr [0], e: arr [1] }
			}

		}

		pos_ops! (PosGeo: Debug);
		pos_ops! (PosGeo: Add, Neg, Rem, Sub);

	}

	mod row_col {

		use super::*;

		#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct PosRowCol <Val> { pub row: Val, pub col: Val }

		impl <Val: Int> PosRowCol <Val> {
			pub const ZERO: Self = Self { row: Val::ZERO, col: Val::ZERO };
		}

		impl <Val: Int> Coord <2> for PosRowCol <Val> {

			type Val = Val;
			type Signed = PosRowCol <Val::Signed>;

			#[ inline ]
			fn coord_to_array (self) -> [Val; 2] {
				[ self.row, self.col ]
			}

			#[ inline ]
			fn coord_from_array (arr: [Val; 2]) -> Self {
				Self { row: arr [0], col: arr [1] }
			}

		}

		pos_ops! (PosRowCol: Debug);
		pos_ops! (PosRowCol: Add, Neg, Rem, Sub);

	}

}

mod dim_3 {

	use super::*;

	pub use xyz::PosXYZ;

	mod xyz {

		use super::*;

		#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct PosXYZ <Val> { pub x: Val, pub y: Val, pub z: Val }

		impl <Val: Int> PosXYZ <Val> {
			pub const ZERO: Self = Self { x: Val::ZERO, y: Val::ZERO, z: Val::ZERO };
			pub const MAX: Self = Self { x: Val::MAX, y: Val::MAX, z: Val::MAX };
			pub const MIN: Self = Self { x: Val::MIN, y: Val::MIN, z: Val::MIN };
		}

		impl <Val: Int> Coord <3> for PosXYZ <Val> {

			type Val = Val;
			type Signed = PosGeo <Val::Signed>;

			#[ inline ]
			fn coord_to_array (self) -> [Val; 3] {
				[ self.x, self.y, self.z ]
			}

			#[ inline ]
			fn coord_from_array (arr: [Val; 3]) -> Self {
				Self { x: arr [0], y: arr [1], z: arr [2] }
			}

		}

		pos_ops! (PosXYZ: Debug);
		pos_ops! (PosXYZ: Add, Neg, Rem, Sub);

	}

}
