use super::*;
use std::fmt::{ Debug, Display };

pub trait Int: Clone + Copy + Debug + Display + Eq + Hash + Ord + IntOps {
	type Signed: IntSigned;
	type Unsigned: IntUnsigned;
	const ZERO: Self;
	const ONE: Self;
	const MIN: Self;
	const MAX: Self;
	fn as_signed (self) -> Self::Signed;
	fn as_unsigned (self) -> Self::Unsigned;
	fn unsigned_abs (self) -> Self::Unsigned;
	fn signum (self) -> Self::Signed;
	fn signed_diff (self, other: Self) -> Option <Self::Signed>;
	fn unsigned_diff (self, other: Self) -> Option <Self::Unsigned>;
	fn add_signed (self, other: Self::Signed) -> Option <Self>;
	fn as_usize (self) -> usize;
	fn to_usize (self) -> Option <usize>;
	fn from_usize (val: usize) -> Option <Self>;
}

impl Int for i16 {
	type Signed = i16;
	type Unsigned = u16;
	const ZERO: i16 = 0;
	const ONE: i16 = 1;
	const MIN: i16 = i16::MIN;
	const MAX: i16 = i16::MAX;
	fn as_signed (self) -> i16 { self }
	fn as_unsigned (self) -> u16 { self as u16 }
	fn unsigned_abs (self) -> u16 { i16::unsigned_abs (self) }
	fn signum (self) -> i16 { i16::signum (self) }
	fn signed_diff (self, other: Self) -> Option <i16> { i16::checked_sub (self, other) }
	fn unsigned_diff (self, other: Self) -> Option <u16> {
		(other <= self).then (|| i16::abs_diff (self, other))
	}
	fn add_signed (self, other: i16) -> Option <i16> { i16::checked_add (self, other) }
	fn as_usize (self) -> usize { self as usize }
	fn to_usize (self) -> Option <usize> { (self >= 0).then (|| self as usize) }
	fn from_usize (val: usize) -> Option <Self> { val.try_into ().ok () }
}

impl Int for u16 {
	type Signed = i16;
	type Unsigned = u16;
	const ZERO: u16 = 0;
	const ONE: u16 = 1;
	const MIN: u16 = u16::MIN;
	const MAX: u16 = u16::MAX;
	fn as_signed (self) -> i16 { self as i16 }
	fn as_unsigned (self) -> u16 { self }
	fn unsigned_abs (self) -> u16 { self }
	fn signum (self) -> i16 { if self > 0 { 1 } else { 0 } }
	fn signed_diff (self, other: Self) -> Option <i16> {
		if other < self { (self - other).try_into ().ok () }
		else { (other - self).try_into ().map (|val| i16::neg (val)).ok () }
	}
	fn unsigned_diff (self, other: Self) -> Option <u16> { u16::checked_sub (self, other) }
	fn add_signed (self, other: i16) -> Option <u16> {
		if other >= 0 { u16::checked_add (self, other as u16) }
		else { u16::checked_sub (self, i16::unsigned_abs (other)) }
	}
	fn as_usize (self) -> usize { self as usize }
	fn to_usize (self) -> Option <usize> { Some (self as usize) }
	fn from_usize (val: usize) -> Option <Self> { val.try_into ().ok () }
}

pub trait IntSigned: Int {
	const NEG_ONE: Self::Signed;
}
impl IntSigned for i16 {
	const NEG_ONE: i16 = -1;
}

pub trait IntUnsigned: Int {}
impl IntUnsigned for u16 {}

pub trait IntSized <const BITS: usize>: Int {}

impl IntSized <16> for i16 {}
impl IntSized <16> for u16 {}

pub trait IntOpsRust: Sized
	+ Add <Output = Self> + Div <Output = Self> + Mul <Output = Self> + Rem <Output = Self>
	+ Sub <Output = Self> {}
impl <Val> IntOpsRust for Val where Val: Sized
	+ Add <Output = Self> + Div <Output = Self> + Mul <Output = Self> + Rem <Output = Self>
	+ Sub <Output = Self> {}

pub trait IntOpsSafe: Sized {
	fn safe_add (self, arg: Self) -> Self;
	fn safe_sub (self, arg: Self) -> Self;
}
impl IntOpsSafe for i16 {
	fn safe_add (self, arg: i16) -> i16 { i16::checked_add (self, arg).unwrap () }
	fn safe_sub (self, arg: i16) -> i16 { i16::checked_sub (self, arg).unwrap () }
}
impl IntOpsSafe for u16 {
	fn safe_add (self, arg: u16) -> u16 { u16::checked_add (self, arg).unwrap () }
	fn safe_sub (self, arg: u16) -> u16 { u16::checked_sub (self, arg).unwrap () }
}

pub trait IntOps: IntOpsRust + IntOpsSafe {}
impl <Val> IntOps for Val where Val: IntOpsRust + IntOpsSafe {}
