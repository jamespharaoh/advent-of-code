use super::*;

pub type GridIter <'sto, Storage, Pos, const DIMS: usize> =
	std::iter::Zip <
		GridKeysIter <Pos, DIMS>,
		<& 'sto Storage as GridStorageIntoIter>::Iter,
	>;

pub struct GridKeysIter <Pos: GridPos <DIMS>, const DIMS: usize> {
	origin: [isize; DIMS],
	size: [usize; DIMS],
	cur: [usize; DIMS],
	done: bool,
	phantom: PhantomData <Pos>,
}

impl <Pos: GridPos <DIMS>, const DIMS: usize> GridKeysIter <Pos, DIMS> {
	pub (crate) const fn new (origin: [isize; DIMS], size: [usize; DIMS]) -> Self {
		Self {
			origin,
			size,
			cur: [0_usize; DIMS],
			done: false,
			phantom: PhantomData,
		}
	}
}

impl <Pos: GridPos <DIMS>, const DIMS: usize> Iterator for GridKeysIter <Pos, DIMS> {

	type Item = Pos;

	#[ inline ]
	fn next (& mut self) -> Option <Pos> {
		if self.done { return None }
		let pos = Pos::from_native (self.cur, self.origin);
		for idx in (0 .. DIMS).rev () {
			self.cur [idx] += 1;
			if self.cur [idx] < self.size [idx] { break }
			self.cur [idx] = 0;
			if idx == 0 { self.done = true; }
		}
		pos
	}

}

/// Wrapping iterator which clones items.
///
/// We don't use [`Cloned`](iter::Cloned) from the standard library because it doesn't handle
/// [`Iterator::skip`] the way we would like. Although it says in the documentation that there is
/// no guarantee each element will be processed, it seems like it does so. Instead, we want to
/// completely bypass any elements which aren't required.
///
#[ derive (Clone) ]
pub struct GridStorageClone <Storage> {
	storage: Storage,
}

impl <Storage> GridStorageClone <Storage> {
	pub (crate) const fn new (storage: Storage) -> Self {
		Self { storage }
	}
}

impl <'sto, Storage, Item> Iterator for GridStorageClone <Storage>
	where
		Storage: Iterator <Item = & 'sto Item>,
		Item: Clone + 'sto {

	type Item = Item;

	#[ inline ]
	fn next (& mut self) -> Option <Item> {
		self.storage.next ().cloned ()
	}

	#[ inline ]
	fn nth (& mut self, num: usize) -> Option <Item> {
		self.storage.nth (num).cloned ()
	}

}
