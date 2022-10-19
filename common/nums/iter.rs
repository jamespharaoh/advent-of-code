use super::*;

pub trait IteratorNums: Iterator {

	#[ inline ]
	fn try_sum <Sum> (self) -> NumResult <Sum>
		where
			Self: Sized,
			Self::Item: Int,
			Sum: Int + TryFrom <Self::Item> {
		let mut sum = Sum::ZERO;
		for item in self {
			sum.try_add_assign (item.try_into ().ok ().ok_or (Overflow) ?) ?;
		}
		Ok (sum)
	}

}

impl <Iter: Iterator> IteratorNums for Iter {
}
