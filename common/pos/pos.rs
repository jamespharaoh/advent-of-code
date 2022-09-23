use std::fmt::{ self, Debug };
use std::ops::{ Add, Neg, Rem, Sub };

use aoc_misc::*;
use aoc_nums as nums;
use nums::Int;
use nums::IntSigned;
use nums::NumResult;
use nums::TryAdd;
use nums::TryMul;

pub use gen_pos::GenPos;
pub use gen_pos::GenPosCore;
pub use gen_pos::GenPosOps;
pub use dim_2::Dir2d;
pub use dim_2::DirGeo;
pub use dim_2::PosXY;
pub use dim_2::PosYX;
pub use dim_2::PosGeo;
pub use dim_2::PosRowCol;
pub use dim_2::Turn2d;
pub use dim_3::AxisXYZ;
pub use dim_3::PosXYZ;
pub use dim_4::PosWXYZ;
pub use dim_4::PosXYZT;
pub use dim_4::PosXYZW;

macro_rules! pos_ops {

	( $name:ident : Debug $(, $rest:tt)* ) => {
		impl <Val: Int> Debug for $name <Val> {
			#[ inline ]
			fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
				let array = self.pos_to_array ();
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
		pos_ops! ($name: $($rest),*);
	};

	( $name:ident : Add $(, $rest:tt)* ) => {
		impl <Val: Int, ArgVal: Int> TryAdd <$name <ArgVal>> for $name <Val>
				where Val: TryAdd <ArgVal, Output = Val> {
			type Output = Self;
			#[ inline ]
			fn try_add (self, other: $name <ArgVal>) -> NumResult <Self> {
				let self_array = self.pos_to_array ();
				let other_array = other.pos_to_array ();
				let mut result_array = Self::ZERO.pos_to_array ();
				for idx in 0 .. self_array.len () {
					result_array [idx] = self_array [idx].try_add (other_array [idx]) ?
				}
				Ok (Self::pos_from_array (result_array))
			}
		}
		impl <Val: Int, ArgVal: Int> Add <$name <ArgVal>> for $name <Val>
				where Val: TryAdd <ArgVal, Output = Val> {
			type Output = Self;
			#[ inline ]
			fn add (self, other: $name <ArgVal>) -> Self {
				self.try_add (other).unwrap ()
			}
		}
		pos_ops! ($name: $($rest),*);
	};
	( $name:ident : Mul $(, $rest:tt)* ) => {
		impl <Val: Int + TryMul <Output = Val>> TryMul <Val> for $name <Val> {
			type Output = Self;
			#[ inline ]
			fn try_mul (self, arg: Val) -> NumResult <Self> {
				let self_array = self.pos_to_array ();
				let mut result_array = Self::ZERO.pos_to_array ();
				for idx in 0 .. self_array.len () {
					result_array [idx] = self_array [idx].try_mul (arg) ?;
				}
				Ok (Self::pos_from_array (result_array))
			}
		}
		impl <Val: Int> Mul <Val> for $name <Val> {
			type Output = Self;
			#[ inline ]
			fn mul (self, arg: Val) -> Self {
				let self_array = self.pos_to_array ();
				let mut result_array = Self::ZERO.pos_to_array ();
				for idx in 0 .. self_array.len () {
					result_array [idx] = self_array [idx] * arg;
				}
				Self::pos_from_array (result_array)
			}
		}
		pos_ops! ($name: $($rest),*);
	};
	( $name:ident : Neg $(, $rest:tt)* ) => {
		impl <Val: Int + Neg <Output = Val>> Neg for $name <Val> {
			type Output = Self;
			#[ inline ]
			fn neg (self) -> Self {
				let self_array = self.pos_to_array ();
				let mut result_array = Self::ZERO.pos_to_array ();
				for idx in 0 .. self_array.len () {
					result_array [idx] = - self_array [idx];
				}
				Self::pos_from_array (result_array)
			}
		}
		pos_ops! ($name: $($rest),*);
	};
	( $name:ident : Rem $(, $rest:tt)* ) => {
		impl <Val: Int> Rem for $name <Val> {
			type Output = Self;
			#[ inline ]
			fn rem (self, other: $name <Val>) -> Self {
				let self_array = self.pos_to_array ();
				let other_array = other.pos_to_array ();
				let mut result_array = Self::ZERO.pos_to_array ();
				for idx in 0 .. self_array.len () {
					result_array [idx] = self_array [idx] % other_array [idx];
				}
				Self::pos_from_array (result_array)
			}
		}
		pos_ops! ($name: $($rest),*);
	};
	( $name:ident : Sub $(, $rest:tt)* ) => {
		impl <Val: Int> Sub <$name <Val::Signed>> for $name <Val> {
			type Output = Self;
			#[ inline ]
			fn sub (self, other: $name <Val::Signed>) -> Self {
				let self_array = self.pos_to_array ();
				let other_array = other.pos_to_array ();
				let mut result_array = Self::ZERO.pos_to_array ();
				for idx in 0 .. self_array.len () {
					result_array [idx] =
						self_array [idx].sub_signed (other_array [idx]).unwrap ();
				}
				Self::pos_from_array (result_array)
			}
		}
		pos_ops! ($name: $($rest),*);
	};
	( $name:ident : ) => {};

}

mod gen_pos {

	use super::*;

	pub trait GenPosCore <const DIMS: usize>: Copy + Debug + Eq + Hash + Ord + Sized {

		//type Signed: GenPos <DIMS>;
		type Val: Int;

		fn pos_to_array (self) -> [Self::Val; DIMS];
		fn pos_from_array (array: [Self::Val; DIMS]) -> Self;

		const ZERO: Self;
		const MIN: Self;
		const MAX: Self;

	}

	pub trait GenPosOps <const DIMS: usize>:
		GenPosCore <DIMS> +
		TryAdd <Output = Self> +
		TryMul <Self::Val, Output = Self> {
	}

	impl <Pos, const DIMS: usize> GenPosOps <DIMS> for Pos
		where
			Pos: GenPosCore <DIMS> +
			TryAdd <Output = Self> +
			TryMul <Self::Val, Output = Self> {
	}

	pub trait GenPos <const DIMS: usize>: GenPosOps <DIMS> {

		#[ inline ]
		#[ must_use ]
		fn zero () -> Self {
			Self::pos_from_array ([Self::Val::ZERO; DIMS])
		}

	}

	impl <Pos, const DIMS: usize> GenPos <DIMS> for Pos where Pos: GenPosOps <DIMS> {
	}

}

mod dim_2 {

	use super::*;

	pub use geo::DirGeo;
	pub use geo::PosGeo;
	pub use row_col::Dir2d;
	pub use row_col::PosRowCol;
	pub use xy::PosXY;
	pub use yx::PosYX;

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Turn2d { None, Left, Right, Around }

	mod xy {

		use super::*;

		#[ derive (Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct PosXY <Val> { pub x: Val, pub y: Val }

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

		impl <Val: Int> GenPosCore <2> for PosXY <Val> {

			type Val = Val;
			//type Signed = PosXY <Val::Signed>;

			#[ inline ]
			fn pos_to_array (self) -> [Val; 2] {
				[ self.x, self.y ]
			}

			#[ inline ]
			fn pos_from_array (arr: [Val; 2]) -> Self {
				Self { x: arr [0], y: arr [1] }
			}

			const ZERO: Self = Self { x: Val::ZERO, y: Val::ZERO };
			const MIN: Self = Self { x: Val::MIN, y: Val::MIN };
			const MAX: Self = Self { x: Val::MAX, y: Val::MAX };

		}

		pos_ops! (PosXY: Debug);
		pos_ops! (PosXY: Add, Mul, Neg, Sub, Rem);

	}

	mod yx {

		use super::*;

		#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct PosYX <Val> { pub y: Val, pub x: Val }

		impl <Val: Int> PosYX <Val> {

			#[ inline ]
			#[ must_use ]
			pub const fn new (y: Val, x: Val) -> Self {
				Self { y, x }
			}

			#[ inline ]
			pub fn up (self, num: Val) -> NumResult <Self> {
				Ok (Self { y: Val::sub_2 (self.y, num) ?, x: self.x })
			}

			#[ inline ]
			pub fn down (self, num: Val) -> NumResult <Self> {
				Ok (Self { y: Val::add_2 (self.y, num) ?, x: self.x })
			}

			#[ inline ]
			pub fn left (& self, num: Val) -> NumResult <Self> {
				Ok (Self { y: self.y, x: Val::sub_2 (self.x, num) ? })
			}

			#[ inline ]
			pub fn right (& self, num: Val) -> NumResult <Self> {
				Ok (Self { y: self.y, x: Val::add_2 (self.x, num) ? })
			}

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

		impl <Val: Int> GenPosCore <2> for PosYX <Val> {

			type Val = Val;
			//type Signed = PosYX <Val::Signed>;

			#[ inline ]
			fn pos_to_array (self) -> [Val; 2] {
				[ self.y, self.x ]
			}

			#[ inline ]
			fn pos_from_array (arr: [Val; 2]) -> Self {
				Self { y: arr [0], x: arr [1] }
			}

			const ZERO: Self = Self { x: Val::ZERO, y: Val::ZERO };
			const MIN: Self = Self { x: Val::MIN, y: Val::MIN };
			const MAX: Self = Self { x: Val::MAX, y: Val::MAX };

		}

		impl <Val: Int <Signed = Val> + IntSigned> From <Dir2d> for PosYX <Val> {

			#[ inline ]
			fn from (dir: Dir2d) -> Self {
				match dir {
					Dir2d::Up => Self { y: Val::NEG_ONE, x: Val::ZERO },
					Dir2d::Down => Self { y: Val::ONE, x: Val::ZERO },
					Dir2d::Left => Self { y: Val::ZERO, x: Val::NEG_ONE },
					Dir2d::Right => Self { y: Val::ZERO, x: Val::ONE },
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

		pos_ops! (PosYX: Debug);
		pos_ops! (PosYX: Add, Mul, Neg, Rem, Sub);

	}

	mod geo {

		use super::*;

		#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct PosGeo <Val> { pub n: Val, pub e: Val }

		impl <Val: Int> PosGeo <Val> {

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

		impl <Val: Int> GenPosCore <2> for PosGeo <Val> {

			type Val = Val;
			//type Signed = PosGeo <Val::Signed>;

			#[ inline ]
			fn pos_to_array (self) -> [Val; 2] {
				[ self.n, self.e ]
			}

			#[ inline ]
			fn pos_from_array (arr: [Val; 2]) -> Self {
				Self { n: arr [0], e: arr [1] }
			}

			const ZERO: Self = Self { n: Val::ZERO, e: Val::ZERO };
			const MIN: Self = Self { n: Val::MIN, e: Val::MIN };
			const MAX: Self = Self { n: Val::MAX, e: Val::MAX };

		}

		pos_ops! (PosGeo: Debug);
		pos_ops! (PosGeo: Add, Mul, Neg, Rem, Sub);

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

	mod row_col {

		use super::*;

		#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct PosRowCol <Val> { pub row: Val, pub col: Val }

		impl <Val: Int> PosRowCol <Val> {

			#[ inline ]
			#[ must_use ]
			pub const fn new (row: Val, col: Val) -> Self {
				Self { row, col }
			}

			#[ inline ]
			#[ must_use ]
			pub fn up (self, num: Val) -> Self {
				Self { row: self.row.safe_sub (num), col: self.col }
			}

			#[ inline ]
			#[ must_use ]
			pub fn down (self, num: Val) -> Self {
				Self { row: self.row.safe_add (num), col: self.col }
			}

			#[ inline ]
			#[ must_use ]
			pub fn left (& self, num: Val) -> Self {
				Self { row: self.row, col: self.col.safe_sub (num) }
			}

			#[ inline ]
			#[ must_use ]
			pub fn right (& self, num: Val) -> Self {
				Self { row: self.row, col: self.col.safe_add (num) }
			}

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

		impl <Val: Int> GenPosCore <2> for PosRowCol <Val> {

			type Val = Val;
			//type Signed = PosRowCol <Val::Signed>;

			#[ inline ]
			fn pos_to_array (self) -> [Val; 2] {
				[ self.row, self.col ]
			}

			#[ inline ]
			fn pos_from_array (arr: [Val; 2]) -> Self {
				Self { row: arr [0], col: arr [1] }
			}

			const ZERO: Self = Self::new (Val::ZERO, Val::ZERO);
			const MIN: Self = Self::new (Val::MIN, Val::MIN);
			const MAX: Self = Self::new (Val::MAX, Val::MAX);

		}

		pos_ops! (PosRowCol: Debug);
		pos_ops! (PosRowCol: Add, Mul, Neg, Rem, Sub);

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

mod dim_3 {

	use super::*;

	pub use xyz::PosXYZ;

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum AxisXYZ { X, Y, Z }

	mod xyz {

		use super::*;

		#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct PosXYZ <Val> { pub x: Val, pub y: Val, pub z: Val }

		impl <Val: Int> PosXYZ <Val> {

			#[ inline ]
			#[ must_use ]
			pub const fn new (x: Val, y: Val, z: Val) -> Self {
				Self { x, y, z }
			}

			pub const ZERO: Self = Self { x: Val::ZERO, y: Val::ZERO, z: Val::ZERO };
			pub const MAX: Self = Self { x: Val::MAX, y: Val::MAX, z: Val::MAX };
			pub const MIN: Self = Self { x: Val::MIN, y: Val::MIN, z: Val::MIN };

		}

		impl <Val: Int> GenPosCore <3> for PosXYZ <Val> {

			type Val = Val;
			//type Signed = PosXYZ <Val::Signed>;

			#[ inline ]
			fn pos_to_array (self) -> [Val; 3] {
				[ self.x, self.y, self.z ]
			}

			#[ inline ]
			fn pos_from_array (arr: [Val; 3]) -> Self {
				Self { x: arr [0], y: arr [1], z: arr [2] }
			}

			const ZERO: Self = Self::new (Val::ZERO, Val::ZERO, Val::ZERO);
			const MIN: Self = Self::new (Val::MIN, Val::MIN, Val::MIN);
			const MAX: Self = Self::new (Val::MAX, Val::MAX, Val::MAX);

		}

		impl <Val: Int> Index <AxisXYZ> for PosXYZ <Val> {

			type Output = Val;

			#[ inline ]
			fn index (& self, axis: AxisXYZ) -> & Val {
				match axis {
					AxisXYZ::X => & self.x,
					AxisXYZ::Y => & self.y,
					AxisXYZ::Z => & self.z,
				}
			}

		}

		impl <Val: Int> IndexMut <AxisXYZ> for PosXYZ <Val> {

			#[ inline ]
			fn index_mut (& mut self, axis: AxisXYZ) -> & mut Val {
				match axis {
					AxisXYZ::X => & mut self.x,
					AxisXYZ::Y => & mut self.y,
					AxisXYZ::Z => & mut self.z,
				}
			}

		}

		pos_ops! (PosXYZ: Debug);
		pos_ops! (PosXYZ: Add, Mul, Neg, Rem, Sub);

	}

}

mod dim_4 {

	use super::*;

	pub use wxyz::PosWXYZ;
	pub use xyzt::PosXYZT;
	pub use xyzw::PosXYZW;

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum AxisWXYZ { W, X, Y, Z }

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum AxisXYZT { X, Y, Z, T }

	mod wxyz {

		use super::*;

		#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct PosWXYZ <Val> { pub w: Val, pub x: Val, pub y: Val, pub z: Val }

		impl <Val: Int> PosWXYZ <Val> {

			#[ inline ]
			pub const fn new (w: Val, x: Val, y: Val, z: Val) -> Self {
				Self { w, x, y, z }
			}

		}

		impl <Val: Int> GenPosCore <4> for PosWXYZ <Val> {

			type Val = Val;
			//type Signed = PosWXYZ <Val::Signed>;

			#[ inline ]
			fn pos_to_array (self) -> [Val; 4] {
				[ self.w, self.x, self.y, self.z ]
			}

			#[ inline ]
			fn pos_from_array (arr: [Val; 4]) -> Self {
				Self { w: arr [0], x: arr [1], y: arr [2], z: arr [3] }
			}

			const ZERO: Self = Self::new (Val::ZERO, Val::ZERO, Val::ZERO, Val::ZERO);
			const MIN: Self = Self::new (Val::MIN, Val::MIN, Val::MIN, Val::MIN);
			const MAX: Self = Self::new (Val::MAX, Val::MAX, Val::MAX, Val::MAX);

		}

		impl <Val: Int> Index <AxisWXYZ> for PosWXYZ <Val> {

			type Output = Val;

			#[ inline ]
			fn index (& self, axis: AxisWXYZ) -> & Val {
				match axis {
					AxisWXYZ::W => & self.w,
					AxisWXYZ::X => & self.x,
					AxisWXYZ::Y => & self.y,
					AxisWXYZ::Z => & self.z,
				}
			}

		}

		impl <Val: Int> IndexMut <AxisWXYZ> for PosWXYZ <Val> {

			#[ inline ]
			fn index_mut (& mut self, axis: AxisWXYZ) -> & mut Val {
				match axis {
					AxisWXYZ::W => & mut self.w,
					AxisWXYZ::X => & mut self.x,
					AxisWXYZ::Y => & mut self.y,
					AxisWXYZ::Z => & mut self.z,
				}
			}

		}

		pos_ops! (PosWXYZ: Debug);
		pos_ops! (PosWXYZ: Add, Mul, Neg, Rem, Sub);

	}

	mod xyzt {

		use super::*;

		#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct PosXYZT <Val> { pub x: Val, pub y: Val, pub z: Val, pub t: Val }

		impl <Val: Int> PosXYZT <Val> {

			#[ inline ]
			#[ must_use ]
			pub const fn new (x: Val, y: Val, z: Val, t: Val) -> Self {
				Self { x, y, z, t }
			}

		}

		impl <Val: Int> GenPosCore <4> for PosXYZT <Val> {

			type Val = Val;
			//type Signed = PosXYZT <Val::Signed>;

			#[ inline ]
			fn pos_to_array (self) -> [Val; 4] {
				[ self.x, self.y, self.z, self.t ]
			}

			#[ inline ]
			fn pos_from_array (arr: [Val; 4]) -> Self {
				Self { x: arr [0], y: arr [1], z: arr [2], t: arr [3] }
			}

			const ZERO: Self = Self::new (Val::ZERO, Val::ZERO, Val::ZERO, Val::ZERO);
			const MIN: Self = Self::new (Val::MIN, Val::MIN, Val::MIN, Val::MIN);
			const MAX: Self = Self::new (Val::MAX, Val::MAX, Val::MAX, Val::MAX);

		}

		impl <Val: Int> Index <AxisXYZT> for PosXYZT <Val> {

			type Output = Val;

			#[ inline ]
			fn index (& self, axis: AxisXYZT) -> & Val {
				match axis {
					AxisXYZT::X => & self.x,
					AxisXYZT::Y => & self.y,
					AxisXYZT::Z => & self.z,
					AxisXYZT::T => & self.t,
				}
			}

		}

		impl <Val: Int> IndexMut <AxisXYZT> for PosXYZT <Val> {

			#[ inline ]
			fn index_mut (& mut self, axis: AxisXYZT) -> & mut Val {
				match axis {
					AxisXYZT::X => & mut self.x,
					AxisXYZT::Y => & mut self.y,
					AxisXYZT::Z => & mut self.z,
					AxisXYZT::T => & mut self.t,
				}
			}

		}

		pos_ops! (PosXYZT: Debug);
		pos_ops! (PosXYZT: Add, Mul, Neg, Rem, Sub);

	}

	mod xyzw {

		use super::*;

		#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub struct PosXYZW <Val> { pub x: Val, pub y: Val, pub z: Val, pub w: Val }

		impl <Val: Int> PosXYZW <Val> {

			#[ inline ]
			#[ must_use ]
			pub const fn new (x: Val, y: Val, z: Val, w: Val) -> Self {
				Self { x, y, z, w }
			}

		}

		impl <Val: Int> GenPosCore <4> for PosXYZW <Val> {

			type Val = Val;
			//type Signed = PosXYZT <Val::Signed>;

			#[ inline ]
			fn pos_to_array (self) -> [Val; 4] {
				[ self.x, self.y, self.z, self.w ]
			}

			#[ inline ]
			fn pos_from_array (arr: [Val; 4]) -> Self {
				Self { x: arr [0], y: arr [1], z: arr [2], w: arr [3] }
			}

			const ZERO: Self = Self::new (Val::ZERO, Val::ZERO, Val::ZERO, Val::ZERO);
			const MIN: Self = Self::new (Val::MIN, Val::MIN, Val::MIN, Val::MIN);
			const MAX: Self = Self::new (Val::MAX, Val::MAX, Val::MAX, Val::MAX);

		}

		impl <Val: Int> Index <AxisWXYZ> for PosXYZW <Val> {

			type Output = Val;

			#[ inline ]
			fn index (& self, axis: AxisWXYZ) -> & Val {
				match axis {
					AxisWXYZ::X => & self.x,
					AxisWXYZ::Y => & self.y,
					AxisWXYZ::Z => & self.z,
					AxisWXYZ::W => & self.w,
				}
			}

		}

		impl <Val: Int> IndexMut <AxisWXYZ> for PosXYZW <Val> {

			#[ inline ]
			fn index_mut (& mut self, axis: AxisWXYZ) -> & mut Val {
				match axis {
					AxisWXYZ::X => & mut self.x,
					AxisWXYZ::Y => & mut self.y,
					AxisWXYZ::Z => & mut self.z,
					AxisWXYZ::W => & mut self.w,
				}
			}

		}

		pos_ops! (PosXYZW: Debug);
		pos_ops! (PosXYZW: Add, Mul, Neg, Rem, Sub);

	}

}
