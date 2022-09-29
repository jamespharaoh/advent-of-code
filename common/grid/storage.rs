use super::*;

/// Trait for backing stores for a [`Grid`]
///
/// This provides a simple abstraction over a fixed size array of items. It is implemented for
/// [`Vec`] and [`BitVec`].
///
pub trait GridStorage {

	type Item;

	fn storage_get (& self, idx: usize) -> Option <Self::Item>;
	fn storage_set (& mut self, idx: usize, item: Self::Item);
	fn storage_len (& self) -> usize;

}

impl <Item, const LEN: usize> GridStorage for [Item; LEN] where Item: Clone {
	type Item = Item;

	#[ inline ]
	fn storage_get (& self, idx: usize) -> Option <Item> {
		self.get (idx).cloned ()
	}

	#[ inline ]
	fn storage_set (& mut self, idx: usize, item: Self::Item) {
		self [idx] = item;
	}

	#[ inline ]
	fn storage_len (& self) -> usize {
		self.len ()
	}

}

impl <Item> GridStorage for Vec <Item> where Item: Clone {
	type Item = Item;

	#[ inline ]
	fn storage_get (& self, idx: usize) -> Option <Item> {
		self.get (idx).cloned ()
	}

	#[ inline ]
	fn storage_set (& mut self, idx: usize, item: Self::Item) {
		self [idx] = item;
	}

	#[ inline ]
	fn storage_len (& self) -> usize {
		self.len ()
	}

}

impl <Item, Encoding> GridStorage for BitVec <Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {

	type Item = Item;

	#[ inline ]
	fn storage_get (& self, idx: usize) -> Option <Item> {
		self.get (idx)
	}

	#[ inline ]
	fn storage_set (& mut self, idx: usize, item: Self::Item) {
		self.set (idx, item);
	}

	#[ inline ]
	fn storage_len (& self) -> usize {
		self.len ()
	}

}

/// Additional trait for backing stores which which can provide references to items
///
pub trait GridStorageMut: GridStorage {
	fn storage_ref (& self, idx: usize) -> Option <& Self::Item>;
	fn storage_mut (& mut self, idx: usize) -> Option <& mut Self::Item>;
}

impl <Item> GridStorageMut for Vec <Item> where Item: Clone {

	#[ inline ]
	fn storage_ref (& self, idx: usize) -> Option <& Item> {
		self.get (idx)
	}

	#[ inline ]
	fn storage_mut (& mut self, idx: usize) -> Option <& mut Item> {
		self.get_mut (idx)
	}

}

/// Extra trait for [`GridStorage`] to support iteration.
///
/// This is a separate trait to make the lifetimes work. It should be implemented on a reference to
/// the storage, rather than directly. This allows us to capture the lifetime without polluting the
/// main trait.
///
pub trait GridStorageIntoIter {

	type Item;
	type Iter: Iterator <Item = Self::Item>;

	fn storage_iter (& self) -> Self::Iter;

}

impl <'sto, Item> GridStorageIntoIter for & 'sto Vec <Item> where Item: Clone {

	type Item = Item;
	type Iter = GridStorageClone <SliceIter <'sto, Item>>;

	#[ inline ]
	fn storage_iter (& self) -> Self::Iter {
		GridStorageClone::new (self.iter ())
	}

}

impl <'sto, Item, Encoding> GridStorageIntoIter for & 'sto BitVec <Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {

	type Item = Item;
	type Iter = BitVecIter <'sto, Item, Encoding>;

	#[ inline ]
	fn storage_iter (& self) -> Self::Iter {
		self.iter ()
	}

}
