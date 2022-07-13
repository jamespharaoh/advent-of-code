use super::*;
use std::fmt::{ Debug, Display };

pub trait SeqNext { fn next (self) -> Self; }
impl SeqNext for i16 { fn next (self) -> i16 { self + 1 } }

pub trait SeqPrev { fn prev (self) -> Self; }
impl SeqPrev for i16 { fn prev (self) -> i16 { self - 1 } }

pub trait ConstZero { const ZERO: Self; }
impl ConstZero for i16 { const ZERO: i16 = 0i16; }

pub trait ConstOne { const ONE: Self; }
impl ConstOne for i16 { const ONE: i16 = 1i16; }

pub trait ConstMin { const MIN: Self; }
impl ConstMin for i16 { const MIN: i16 = i16::MIN; }

pub trait ConstMax { const MAX: Self; }
impl ConstMax for i16 { const MAX: i16 = i16::MAX; }

pub trait IntConst: ConstMax + ConstMin + ConstOne + ConstZero {}
impl <Val> IntConst for Val where Val: ConstMax + ConstMin + ConstOne + ConstZero {}

pub trait IntOps: Sized
	+ Add <Output = Self> + Sub <Output = Self>
	+ Mul <Output = Self> + Div <Output = Self> {}
impl <Val> IntOps for Val where Val: Sized
	+ Add <Output = Self> + Sub <Output = Self>
	+ Mul <Output = Self> + Div <Output = Self> {}

pub trait IntSeq: SeqNext + SeqPrev {}
impl <Val> IntSeq for Val where Val: SeqNext + SeqPrev {}

pub trait IntBase: Clone + Copy + Debug + Display + Eq + Hash + Ord {}
impl <Val> IntBase for Val where Val: Clone + Copy + Debug + Display + Eq + Hash + Ord {}

pub trait IntAs {
	fn as_usize (self) -> usize;
}
impl IntAs for i16 {
	fn as_usize (self) -> usize { self as usize }
}

pub trait IntTo {
	fn to_usize (self) -> Option <usize>;
}
impl IntTo for i16 {
	fn to_usize (self) -> Option <usize> { (self >= 0).then (|| self as usize) }
}

pub trait IntFrom: Sized {
	fn from_usize (val: usize) -> Option <Self>;
}
impl IntFrom for i16 {
	fn from_usize (val: usize) -> Option <Self> { (val < i16::MAX as usize).then (|| val as i16) }
}

pub trait Int: IntAs + IntBase + IntConst + IntFrom + IntOps + IntSeq + IntTo {}
impl Int for i16 {}

pub trait Num: Int {}
impl <Val> Num for Val where Val: Int {}
