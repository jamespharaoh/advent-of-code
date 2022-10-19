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

	#[ inline ]
	fn max_ok_by_key <Item, Error, Key, KeyFn> (mut self, mut key_fn: KeyFn) -> Result <Option <Item>, Error>
		where
			Key: Ord,
			KeyFn: FnMut (& Item) -> Key,
			Self: Sized + Iterator <Item = Result <Item, Error>> {
		let (mut max_key, mut max_item) = if let Some (item) = self.next () {
			let item = item ?;
			(key_fn (& item), item)
		} else { return Ok (None) };
		for item in self {
			let item = item ?;
			let key = key_fn (& item);
			if key <= max_key { continue }
			max_key = key;
			max_item = item;
		}
		Ok (Some (max_item))
	}

}

impl <SomeIter: Iterator> IteratorExt for SomeIter {}
