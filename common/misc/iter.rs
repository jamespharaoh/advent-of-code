use super::prelude::*;

pub trait IteratorExt: Iterator {

	#[ inline ]
	fn array <const DIM: usize> (mut self) -> [Self::Item; DIM]
			where Self: Sized, Self::Item: Copy + Default {
		let mut result = [default (); DIM];
		for idx in 0 .. DIM {
			result [idx] = self.next ().unwrap ();
		}
		assert! (self.next ().is_none ());
		result
	}

	#[ inline ]
	fn try_array <Item, Error, const DIM: usize> (mut self) -> Result <[Item; DIM], Error>
			where Self: Sized + Iterator <Item = Result <Item, Error>>, Item: Copy + Default {
		let mut result = [default (); DIM];
		for idx in 0 .. DIM {
			result [idx] = self.next ().unwrap () ?;
		}
		assert! (self.next ().is_none ());
		Ok (result)
	}

}

impl <SomeIter: Iterator> IteratorExt for SomeIter {}
