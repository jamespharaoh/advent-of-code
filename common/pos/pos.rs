use std::fmt::{ self, Debug };
use std::ops::{ Add, Neg, Rem, Sub };

use aoc_checked::checked as chk;
use aoc_misc::prelude::*;
use aoc_nums as nums;
use nums::Int;
use nums::IntSigned;
use nums::NumResult;
use nums::TryAdd;
use nums::TryMul;
use nums::TrySub;

pub use gen::GenAxis;
pub use gen::GenPos;
pub use dim_2::AxisRowCol;
pub use dim_2::AxisXY;
pub use dim_2::Dir2d;
pub use dim_2::DirGeo;
pub use dim_2::DirGeoHexLat;
pub use dim_2::PosXY;
pub use dim_2::PosYX;
pub use dim_2::PosGeo;
pub use dim_2::PosGeoHexLat;
pub use dim_2::PosRowCol;
pub use dim_2::Turn2d;
pub use dim_2::TurnHex;

macro_rules! pos_decl {
	(
		$name:ident $axis_type:ident $dims:literal
		$(, $field:ident $axis:ident $neg:ident $pos:ident )+
	) => {

		#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
		pub enum $axis_type { $($axis),+ }

		impl GenAxis <$dims> for $axis_type { }

		#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct $name <Val> { $(pub $field: Val),+ }

		impl <Val: Int> $name <Val> {

			#[ inline ]
			#[ must_use ]
			pub const fn new ($($field: Val),+) -> Self {
				Self { $($field),+ }
			}

			pub const ZERO: Self = Self { $($field: Val::ZERO),+ };
			pub const MIN: Self = Self { $($field: Val::MIN),+ };
			pub const MAX: Self = Self { $($field: Val::MAX),+ };

			$(

				#[ inline ]
				pub fn $neg (self, num: Val) -> NumResult <Self> {
					let mut result = self;
					result.$field = Val::sub_2 (result.$field, num) ?;
					Ok (result)
				}

				#[ inline ]
				pub fn $pos (self, num: Val) -> NumResult <Self> {
					let mut result = self;
					result.$field = Val::add_2 (result.$field, num) ?;
					Ok (result)
				}

			)+

		}

		impl <Val: Int> GenPos <$dims> for $name <Val> {

			type Val = Val;
			type Axis = $axis_type;

			const ZERO: Self = Self { $($field: Val::ZERO),+ };
			const MIN: Self = Self { $($field: Val::MIN),+ };
			const MAX: Self = Self { $($field: Val::MAX),+ };

		}

		impl <Val: Int> From <[Val; $dims]> for $name <Val> {
			#[ inline ]
			fn from (array: [Val; $dims]) -> Self {
				let [ $($field),+ ] = array;
				Self { $($field),+ }
			}
		}

		impl <Val: Int> From <$name <Val>> for [Val; $dims] {
			#[ inline ]
			fn from (pos: $name <Val>) -> Self {
				[ $(pos.$field),+ ]
			}
		}

		impl <Val: Int> Index <$axis_type> for $name <Val> {

			type Output = Val;

			#[ inline ]
			fn index (& self, axis: $axis_type) -> & Val {
				match axis {
					$( $axis_type::$axis => & self.$field, )+
				}
			}

		}

		impl <Val: Int> IndexMut <$axis_type> for $name <Val> {

			#[ inline ]
			fn index_mut (& mut self, axis: $axis_type) -> & mut Val {
				match axis {
					$( $axis_type::$axis => & mut self.$field, )+
				}
			}

		}

		pos_ops! ($name <$dims>: Debug, Default);
		pos_ops! ($name <$dims>: Add, Mul, Neg, Rem, Sub);

	};
}

macro_rules! pos_dirs {

	(
		$name:ident $dir:ident $dims:literal
		$(, $variant:ident $method:ident [ $($sign:ident),+ ] )+
		$(,)?
	) => {

		impl <Val: Int <Signed = Val> + IntSigned> From <$dir> for $name <Val> {
			#[ inline ]
			fn from (dir: $dir) -> Self {
				match dir {
					$( $dir::$variant => Self::new ($(Val::$sign),+), )+
				}
			}
		}

	};

}

macro_rules! pos_ops {

	( $name:ident <$dims:literal>: Debug $(,$rest:ident)*) => {

		impl <Val: Int> Debug for $name <Val> {
			#[ inline ]
			fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
				let array: [Val; $dims] = (* self).into ();
				formatter.write_str (stringify! ($name)) ?;
				formatter.write_str (" (") ?;
				for idx in 0 .. array.len () {
					if idx != 0 { formatter.write_str (", ") ?; }
					Debug::fmt (& array [idx], formatter) ?;
				}
				formatter.write_str (")") ?;
				Ok (())
			}
		}

		pos_ops! ($name <$dims>: $($rest),*);

	};

	( $name:ident <$dims:literal>: Default $(,$rest:ident)*) => {

		impl <Val: Int> Default for $name <Val> {
			#[ inline ]
			fn default () -> Self {
				Self::ZERO
			}
		}

	};

	( $name:ident <$dims:literal>: Add $(,$rest:ident)* ) => {

		impl <Val: Int, ArgVal: Int> Add <$name <ArgVal>> for $name <Val>
				where Val: TryAdd <ArgVal, Output = Val> {
			type Output = Self;
			#[ inline ]
			fn add (self, other: $name <ArgVal>) -> Self {
				self.try_add (other).unwrap ()
			}
		}

		impl <Val: Int, ArgVal: Int> AddAssign <$name <ArgVal>> for $name <Val>
				where Val: TryAdd <ArgVal, Output = Val> {
			#[ inline ]
			fn add_assign (& mut self, other: $name <ArgVal>) {
				* self = self.try_add (other).unwrap ()
			}
		}

		impl <Val: Int, ArgVal: Int> TryAdd <$name <ArgVal>> for $name <Val>
				where Val: TryAdd <ArgVal, Output = Val> {
			type Output = Self;
			#[ inline ]
			fn try_add (self, other: $name <ArgVal>) -> NumResult <Self> {
				let self_array: [Val; $dims] = self.into ();
				let other_array: [ArgVal; $dims] = other.into ();
				let mut result_array = [Val::ZERO; $dims];
				for idx in 0 .. self_array.len () {
					result_array [idx] = self_array [idx].try_add (other_array [idx]) ?
				}
				Ok (result_array.into ())
			}
		}

		pos_ops! ($name <$dims>: $($rest),*);

	};

	( $name:ident <$dims:literal>: Mul $(,$rest:ident)* ) => {

		impl <Val: Int + TryMul <Output = Val>> TryMul <Val> for $name <Val> {
			type Output = Self;
			#[ inline ]
			fn try_mul (self, arg: Val) -> NumResult <Self> {
				let self_array: [Val; $dims] = self.into ();
				let mut result_array = [Val::ZERO; $dims];
				for idx in 0 .. self_array.len () {
					result_array [idx] = self_array [idx].try_mul (arg) ?;
				}
				Ok (result_array.into ())
			}
		}

		impl <Val: Int> Mul <Val> for $name <Val> {
			type Output = Self;
			#[ inline ]
			fn mul (self, arg: Val) -> Self {
				let self_array: [Val; $dims] = self.into ();
				let mut result_array = [Val::ZERO; $dims];
				for idx in 0 .. self_array.len () {
					result_array [idx] = self_array [idx] * arg;
				}
				result_array.into ()
			}
		}

		pos_ops! ($name <$dims>: $($rest),*);

	};

	( $name:ident <$dims:literal>: Neg $(,$rest:ident)* ) => {

		impl <Val: Int + Neg <Output = Val>> Neg for $name <Val> {
			type Output = Self;
			#[ inline ]
			fn neg (self) -> Self {
				let self_array: [Val; $dims] = self.into ();
				let mut result_array = [Val::ZERO; $dims];
				for idx in 0 .. self_array.len () {
					result_array [idx] = - self_array [idx];
				}
				result_array.into ()
			}
		}

		pos_ops! ($name <$dims>: $($rest),*);

	};

	( $name:ident <$dims:literal>: Rem $(,$rest:ident)* ) => {

		impl <Val: Int> Rem for $name <Val> {
			type Output = Self;
			#[ inline ]
			fn rem (self, other: $name <Val>) -> Self {
				let self_array: [Val; $dims] = self.into ();
				let other_array: [Val; $dims] = other.into ();
				let mut result_array = [Val::ZERO; $dims];
				for idx in 0 .. self_array.len () {
					result_array [idx] = self_array [idx] % other_array [idx];
				}
				result_array.into ()
			}
		}

		pos_ops! ($name <$dims>: $($rest),*);

	};

	( $name:ident <$dims:literal>: Sub $(,$rest:ident)* ) => {

		impl <Val: Int, ArgVal: Int> Sub <$name <ArgVal>> for $name <Val>
				where Val: TrySub <ArgVal, Output = Val> {
			type Output = Self;
			#[ inline ]
			fn sub (self, other: $name <ArgVal>) -> Self {
				self.try_sub (other).unwrap ()
			}
		}

		impl <Val: Int, ArgVal: Int> TrySub <$name <ArgVal>> for $name <Val>
				where Val: TrySub <ArgVal, Output = Val> {
			type Output = Self;
			#[ inline ]
			fn try_sub (self, other: $name <ArgVal>) -> NumResult <Self> {
				let self_array: [Val; $dims] = self.into ();
				let other_array: [ArgVal; $dims] = other.into ();
				let mut result_array = [Val::ZERO; $dims];
				for idx in 0 .. self_array.len () {
					result_array [idx] = self_array [idx].try_sub (other_array [idx]) ?
				}
				Ok (result_array.into ())
			}
		}

		pos_ops! ($name <$dims>: $($rest),*);

	};

	( $name:ident <$dims:literal>: ) => {};

}

mod gen {

	use super::*;

	pub trait GenAxis <const DIMS: usize> {
	}

	pub trait GenPos <const DIMS: usize>:
		Copy + Debug + Eq + Hash + Ord + Sized +
		From <[Self::Val; DIMS]> +
		Into <[Self::Val; DIMS]> +
		TryAdd <Output = Self> +
		TryMul <Self::Val, Output = Self> {

		type Val: Int;
		type Axis: GenAxis <DIMS>;

		#[ inline ]
		#[ must_use ]
		fn zero () -> Self {
			[Self::Val::ZERO; DIMS].into ()
		}

		#[ inline ]
		fn as_array (self) -> [Self::Val; DIMS] {
			self.into ()
		}

		#[ inline ]
		fn into_iter (self) -> array::IntoIter <Self::Val, DIMS> {
			self.as_array ().into_iter ()
		}

		const ZERO: Self;
		const MIN: Self;
		const MAX: Self;

	}

}

mod dim_2 {

	use super::*;

	pub use geo::*;
	pub use geo_hex::*;
	pub use row_col::*;
	pub use xy::*;
	pub use yx::*;

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Turn2d { None, Left, Right, Around }

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum TurnHex { None, SoftRight, HardRight, Around, HardLeft, SoftLeft }

	mod xy {

		use super::*;

		pos_decl! (PosXY AxisXY 2, x X left right, y Y down up);

		impl <Val: Int> PosXY <Val> {

			#[ inline ]
			pub fn adjacent_4 (& self) -> ArrayVec <Self, 4> {
				let mut result = ArrayVec::new ();
				let Self { x, y } = * self;
				if self.x > Val::MIN { result.push (Self { x: x - Val::ONE, y }); }
				if self.x < Val::MAX { result.push (Self { x: x + Val::ONE, y }); }
				if self.y > Val::MIN { result.push (Self { x, y: y - Val::ONE }); }
				if self.y < Val::MAX { result.push (Self { x, y: y + Val::ONE }); }
				result
			}

		}

	}

	mod yx {

		use super::*;

		pos_decl! (PosYX AxisXY 2, y Y up down, x X left right);

		pos_dirs! {
			PosYX Dir2d 2,
			Up up [ NEG_ONE, ZERO ],
			Down down [ ONE, ZERO ],
			Left left [ ZERO, NEG_ONE ],
			Right right [ ZERO, ONE ],
		}

		impl <Val: Int> PosYX <Val> {

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

		impl <Val: IntSigned> Add <Turn2d> for PosYX <Val> {
			type Output = Self;

			#[ inline ]
			fn add (self, turn: Turn2d) -> Self {
				match turn {
					Turn2d::None => Self::new (self.y, self.x),
					Turn2d::Right => Self::new (self.x, - self.y),
					Turn2d::Around => Self::new (- self.y, - self.x),
					Turn2d::Left => Self::new (- self.x, self.y),
				}
			}

		}

		impl <Val: Int> Index <AxisXY> for PosXYZ <Val> {

			type Output = Val;

			#[ inline ]
			fn index (& self, axis: AxisXY) -> & Val {
				match axis {
					AxisXY::X => & self.x,
					AxisXY::Y => & self.y,
				}
			}

		}

		impl <Val: Int> TryAdd <(Dir2d, Val)> for PosYX <Val> {

			type Output = Self;

			#[ inline ]
			fn try_add (self, (dir, dist): (Dir2d, Val)) -> NumResult <Self> {
				let mut result = self;
				match dir {
					Dir2d::Up => result.y = Val::sub_2 (result.y, dist) ?,
					Dir2d::Down => result.y = Val::add_2 (result.y, dist) ?,
					Dir2d::Left => result.x = Val::sub_2 (result.x, dist) ?,
					Dir2d::Right => result.x = Val::add_2 (result.x, dist) ?,
				}
				Ok (result)
			}

		}

	}

	mod geo {

		use super::*;

		pos_decl! (PosGeo AxisGeo 2, n N south north, e E west east);
		pos_dirs! (PosGeo DirGeo 2,
			South south [ NEG_ONE, ZERO ],
			North north [ ONE, ZERO ],
			West west [ ZERO, NEG_ONE ],
			East east [ ZERO, ONE ]);

		impl <Val: Int> PosGeo <Val> {

			#[ inline ]
			#[ must_use ]
			pub fn left (& self) -> Self where Val: IntSigned {
				Self { n: self.e, e: - self.n }
			}

			#[ inline ]
			#[ must_use ]
			pub fn right (& self) -> Self where Val: IntSigned {
				Self { n: - self.e, e: self.n }
			}

			#[ inline ]
			#[ must_use ]
			pub fn around (& self) -> Self where Val: IntSigned {
				Self { n: - self.n, e: - self.e }
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

		#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
		pub enum DirGeo { North, South, East, West }

		impl DirGeo {

			#[ inline ]
			#[ must_use ]
			pub fn left (self) -> Self {
				self + Turn2d::Left
			}

			#[ inline ]
			#[ must_use ]
			pub fn right (self) -> Self {
				self + Turn2d::Right
			}

			#[ inline ]
			#[ must_use ]
			pub fn around (self) -> Self {
				self + Turn2d::Around
			}

		}

		impl Add <Turn2d> for DirGeo {

			type Output = Self;

			#[ inline ]
			fn add (self, other: Turn2d) -> Self {
				use Turn2d::{ None, Left, Right, Around };
				use DirGeo::{ North, South, East, West };
				match (self, other) {
					(North, None) | (West, Right) | (East, Left) | (South, Around) => North,
					(South, None) | (East, Right) | (West, Left) | (North, Around) => South,
					(East, None) | (North, Right) | (South, Left) | (West, Around) => East,
					(West, None) | (South, Right) | (North, Left) | (East, Around) => West,
				}
			}

		}

		impl <Val: Int> Add <(DirGeo, Val)> for PosGeo <Val> {

			type Output = NumResult <Self>;

			#[ inline ]
			fn add (self, (dir, dist): (DirGeo, Val)) -> NumResult <Self> {
				let mut result = self;
				match dir {
					DirGeo::North => result.n = Val::add_2 (result.n, dist) ?,
					DirGeo::South => result.n = Val::sub_2 (result.n, dist) ?,
					DirGeo::East => result.e = Val::add_2 (result.e, dist) ?,
					DirGeo::West => result.e = Val::sub_2 (result.e, dist) ?,
				}
				Ok (result)
			}

		}

	}

	mod geo_hex {

		use super::*;

		pos_decl! (PosGeoHexLat AxisGeoHexLat 2, nw NW south_east north_west, e E west east);

		impl <Val: Int> PosGeoHexLat <Val> {

			#[ inline ]
			pub fn north_east (& self, num: Val) -> NumResult <Self> {
				Ok (Self { nw: chk! (self.nw + num) ?, e: chk! (self.e + num) ? })
			}

			#[ inline ]
			pub fn south_west (& self, num: Val) -> NumResult <Self> {
				Ok (Self { nw: chk! (self.nw - num) ?, e: chk! (self.e - num) ? })
			}

			#[ inline ]
			pub fn adjacent (& self) -> ArrayVec <Self, 6> where Val: Int {
				[
					self.north_west (Val::ONE).ok (),
					self.north_east (Val::ONE).ok (),
					self.east (Val::ONE).ok (),
					self.south_east (Val::ONE).ok (),
					self.south_west (Val::ONE).ok (),
					self.west (Val::ONE).ok (),
				].into_iter ().flatten ().collect ()
			}

		}

		#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
		pub enum DirGeoHexLat { NorthWest, NorthEast, East, SouthEast, SouthWest, West }

		impl DirGeoHexLat {

			#[ inline ]
			#[ must_use ]
			pub fn soft_right (self) -> Self {
				self + TurnHex::SoftRight
			}

			#[ inline ]
			#[ must_use ]
			pub fn hard_right (self) -> Self {
				self + TurnHex::HardRight
			}

			#[ inline ]
			#[ must_use ]
			pub fn around (self) -> Self {
				self + TurnHex::Around
			}

			#[ inline ]
			#[ must_use ]
			pub fn hard_left (self) -> Self {
				self + TurnHex::HardLeft
			}

			#[ inline ]
			#[ must_use ]
			pub fn soft_left (self) -> Self {
				self + TurnHex::SoftLeft
			}

		}

		impl Add <TurnHex> for DirGeoHexLat {

			type Output = Self;

			#[ inline ]
			fn add (self, other: TurnHex) -> Self {
				use TurnHex::{ None, SoftRight, HardRight, Around, HardLeft, SoftLeft };
				use DirGeoHexLat::{ NorthWest, NorthEast, East, SouthEast, SouthWest, West };
				match (self, other) {
					(NorthWest, None) | (West, SoftRight) | (SouthWest, HardRight) |
						(SouthEast, Around) | (East, HardLeft) | (NorthEast, SoftLeft) =>
							NorthWest,
					(NorthEast, None) | (NorthWest, SoftRight) | (West, HardRight) |
						(SouthWest, Around) | (SouthEast, HardLeft) | (East, SoftLeft) =>
							NorthEast,
					(East, None) | (NorthEast, SoftRight) | (NorthWest, HardRight) |
						(West, Around) | (SouthWest, HardLeft) | (SouthEast, SoftLeft) =>
							East,
					(SouthEast, None) | (East, SoftRight) | (NorthEast, HardRight) |
						(NorthWest, Around) | (West, HardLeft) | (SouthWest, SoftLeft) =>
							SouthEast,
					(SouthWest, None) | (SouthEast, SoftRight) | (East, HardRight) |
						(NorthEast, Around) | (NorthWest, HardLeft) | (West, SoftLeft) =>
							SouthWest,
					(West, None) | (SouthWest, SoftRight) | (SouthEast, HardRight) |
						(East, Around) | (NorthEast, HardLeft) | (NorthWest, SoftLeft) =>
							West,
				}
			}

		}

	}

	mod row_col {

		use super::*;

		pos_decl! (PosRowCol AxisRowCol 2, row Row up down, col Col left right);
		pos_dirs! {
			PosRowCol Dir2d 2,
			Up up [ NEG_ONE, ZERO ],
			Down down [ ONE, ZERO ],
			Left left [ ZERO, NEG_ONE ],
			Right right [ ZERO, ONE ],
		}

		impl <Val: Int> PosRowCol <Val> {

			#[ inline ]
			pub fn adjacent_4 (& self) -> ArrayVec <Self, 4> where Val: Int {
				let mut result = ArrayVec::new ();
				let Self { row, col } = * self;
				if row > Val::MIN { result.push (Self { row: row - Val::ONE, col }); }
				if row < Val::MAX { result.push (Self { row: row + Val::ONE, col }); }
				if col > Val::MIN { result.push (Self { row, col: col - Val::ONE }); }
				if col < Val::MAX { result.push (Self { row, col: col + Val::ONE }); }
				result
			}

			#[ inline ]
			pub fn adjacent_8 (& self) -> ArrayVec <Self, 8> where Val: Int {
				let mut result = ArrayVec::new ();
				let Self { row, col } = * self;
				if self.row > Val::MIN {
					let row = row - Val::ONE;
					if self.col > Val::MIN { result.push (Self { row, col: col - Val::ONE }); }
					result.push (Self { row, col });
					if self.col < Val::MAX { result.push (Self { row, col: col + Val::ONE }); }
				}
				if self.col > Val::MIN { result.push (Self { row, col: col - Val::ONE }); }
				if self.col < Val::MAX { result.push (Self { row, col: col + Val::ONE }); }
				if self.row < Val::MAX {
					let row = row + Val::ONE;
					if self.col > Val::MIN { result.push (Self { row, col: col - Val::ONE }); }
					result.push (Self { row, col });
					if self.col < Val::MAX { result.push (Self { row, col: col + Val::ONE }); }
				}
				result
			}

		}

		#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub enum Dir2d { Up, Down, Left, Right }

		impl Dir2d {

			#[ inline ]
			#[ must_use ]
			pub const fn left (self) -> Self {
				match self {
					Self::Up => Self::Left,
					Self::Down => Self::Right,
					Self::Left => Self::Down,
					Self::Right => Self::Up,
				}
			}

			#[ inline ]
			#[ must_use ]
			pub const fn right (self) -> Self {
				match self {
					Self::Up => Self::Right,
					Self::Down => Self::Left,
					Self::Left => Self::Up,
					Self::Right => Self::Down,
				}
			}

			#[ inline ]
			#[ must_use ]
			pub const fn around (self) -> Self {
				match self {
					Self::Up => Self::Down,
					Self::Down => Self::Up,
					Self::Left => Self::Right,
					Self::Right => Self::Left,
				}
			}

		}

		impl Add <Turn2d> for Dir2d {

			type Output = Self;

			#[ inline ]
			fn add (self, other: Turn2d) -> Self {
				use Turn2d::{ None, Left as TLeft, Right as TRight, Around };
				use Dir2d::{ Up, Down, Left as DLeft, Right as DRight };
				match (self, other) {
					(Up, None) | (DLeft, TRight) | (DRight, TLeft) | (Down, Around) => Up,
					(Down, None) | (DRight, TRight) | (DLeft, TLeft) | (Up, Around) => Down,
					(DLeft, None) | (Down, TRight) | (Up, TLeft) | (DRight, Around) => DLeft,
					(DRight, None) | (Up, TRight) | (Down, TLeft) | (DLeft, Around) => DRight,
				}
			}

		}

		impl <Val: Int> Add <(Dir2d, Val)> for PosRowCol <Val> {

			type Output = NumResult <Self>;

			#[ inline ]
			fn add (self, (dir, dist): (Dir2d, Val)) -> NumResult <Self> {
				let mut result = self;
				match dir {
					Dir2d::Up => result.row = Val::sub_2 (result.row, dist) ?,
					Dir2d::Down => result.row = Val::add_2 (result.row, dist) ?,
					Dir2d::Left => result.col = Val::sub_2 (result.col, dist) ?,
					Dir2d::Right => result.col = Val::add_2 (result.col, dist) ?,
				}
				Ok (result)
			}

		}

	}

}

pos_decl! (PosXYZ AxisXYZ 3, x X left right, y Y down up, z Z behind ahead);

pos_decl! (PosWXYZ AxisWXYZ 4, w W wane wax, x X left right, y Y down up, z Z behind ahead);
pos_decl! (PosXYZT AxisXYZT 4, x X left right, y Y down up, z Z behind ahead, t T before after);
pos_decl! (PosXYZW AxisXYZW 4, x X left right, y Y down up, z Z behind ahead, w W wane wax);
