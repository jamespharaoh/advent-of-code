use super::prelude::*;

#[ cfg (not (fuzzing)) ]
pub use fast_hash::*;

#[ cfg (fuzzing) ]
pub use test_hash::*;

#[ cfg (not (fuzzing)) ]
mod fast_hash {
	pub use ahash::AHashMap as HashMap;
	pub use ahash::AHashSet as HashSet;
}

#[ cfg (fuzzing) ]
mod test_hash {
	pub use std::collections::BTreeSet as HashSet;
	pub use super::test_map::HashMap;
}

#[ cfg (fuzzing) ]
mod test_map;

pub struct MapToIndex <Item> {
	items: Vec <Item>,
	indexes: HashMap <Item, usize>,
}

impl <Item: Clone + Eq + Hash + Ord> MapToIndex <Item> {

	#[ inline ]
	#[ must_use ]
	pub fn new () -> Self {
		Self {
			items: Vec::new (),
			indexes: HashMap::new (),
		}
	}

	#[ inline ]
	#[ must_use ]
	pub fn len (& self) -> usize {
		self.items.len ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn is_empty (& self) -> bool {
		self.items.is_empty ()
	}

	#[ inline ]
	pub fn insert (& mut self, item: Item) -> usize {
		* self.indexes.entry (item.clone ()).or_insert_with (|| {
			let idx = self.items.len ();
			self.items.push (item);
			idx
		})
	}

}

impl <Item: Clone + Eq + Hash + Ord> Default for MapToIndex <Item> {

	#[ inline ]
	fn default () -> Self {
		Self::new ()
	}

}

impl <Item> Deref for MapToIndex <Item> {

	type Target = [Item];

	#[ inline ]
	fn deref (& self) -> & [Item] {
		& self.items
	}

}

impl <Item: Clone + Eq + Hash + Ord> FromIterator <Item> for MapToIndex <Item> {

	#[ inline ]
	fn from_iter <Iter: IntoIterator <Item = Item>> (iter: Iter) -> Self {
		iter.into_iter ()
			.fold (Self::new (), |mut map, item| { map.insert (item); map })
	}

}

impl <Item: Eq + Hash + Ord> Index <& Item> for MapToIndex <Item> {

	type Output = usize;

	#[ inline ]
	fn index (& self, item: & Item) -> & usize {
		self.indexes.get (item).unwrap ()
	}

}
