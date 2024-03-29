use super::*;

pub struct HashMap <Key, Val, Hshr = RandomHasher> {
	map: BTreeMap <Key, Val>,
	phantom: PhantomData <Hshr>,
}

impl <Key, Val> HashMap <Key, Val, RandomHasher> where Key: Ord {

	pub fn new () -> Self {
		Self {
			map: BTreeMap::new (),
			phantom: PhantomData,
		}
	}

	pub fn with_capacity (_capacity: usize) -> Self {
		Self::new ()
	}

}

impl <Key, Val, Hshr> HashMap <Key, Val, Hshr> where Key: Ord {

	pub fn clear (& mut self) {
		self.map.clear ();
	}

	pub fn contains_key <Qry> (& self, key: & Qry) -> bool
		where
			Key: Borrow <Qry>,
			Qry: Eq + Hash + Ord + ?Sized {
		self.map.contains_key (key)
	}

	pub fn get <Qry> (& self, key: & Qry) -> Option <& Val>
		where
			Key: Borrow <Qry>,
			Qry: Eq + Hash + Ord + ?Sized {
		self.map.get (key)
	}

	pub fn get_mut <Qry> (& mut self, key: & Qry) -> Option <& mut Val>
		where
			Key: Borrow <Qry>,
			Qry: Eq + Hash + Ord + ?Sized {
		self.map.get_mut (key)
	}

	pub fn entry (& mut self, key: Key) -> BTreeEntry <'_, Key, Val> {
		self.map.entry (key)
	}

	pub fn insert (& mut self, key: Key, val: Val) -> Option <Val> {
		self.map.insert (key, val)
	}

	pub fn is_empty (& self) -> bool {
		self.map.is_empty ()
	}

	pub fn iter (& self) -> BTreeIter <'_, Key, Val> {
		self.map.iter ()
	}

	pub fn iter_mut (& mut self) -> BTreeIterMut <'_, Key, Val> {
		self.map.iter_mut ()
	}

	pub fn keys (& self) -> BTreeKeys <'_, Key, Val> {
		self.map.keys ()
	}

	pub fn len (& self) -> usize {
		self.map.len ()
	}

	pub fn remove <Qry: ?Sized> (& mut self, value: & Qry) -> Option <Val>
		where
			Key: Borrow <Qry>,
			Qry: Hash + Eq + Ord {
		self.map.remove (value)
	}

	pub fn retain <Func> (& mut self, func: Func)
			where Func: FnMut (& Key, & mut Val) -> bool {
		self.map.retain (func)
	}

	pub fn values (& self) -> BTreeValues <Key, Val> {
		self.map.values ()
	}

}

impl <Key, Val> Clone for HashMap <Key, Val> where Key: Clone, Val: Clone {
	fn clone (& self) -> Self {
		Self {
			map: self.map.clone (),
			phantom: PhantomData,
		}
	}
}

impl <Key, Val> Debug for HashMap <Key, Val>
		where Key: Debug, Val: Debug {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		self.map.fmt (formatter)
	}
}

impl <Key, Val> Default for HashMap <Key, Val> {
	fn default () -> Self {
		Self {
			map: BTreeMap::default (),
			phantom: PhantomData,
		}
	}
}

impl <Key, Val> Eq for HashMap <Key, Val>
	where Key: Eq, Val: Eq {
}

impl <Key, Val> FromIterator <(Key, Val)> for HashMap <Key, Val>
		where Key: Ord {
	fn from_iter <Iter> (iter: Iter) -> Self
			where Iter: IntoIterator <Item = (Key, Val)> {
		Self {
			map: BTreeMap::from_iter (iter),
			phantom: PhantomData,
		}
	}
}

impl <Key, Val, Qry> Index <& '_ Qry> for HashMap <Key, Val>
	where
		Key: Ord + Borrow <Qry>,
		Qry: Ord + ?Sized {
	type Output = Val;
	fn index (& self, query: & Qry) -> & Val {
		self.map.get (query).unwrap ()
	}
}

impl <'map, Key, Val> IntoIterator for HashMap <Key, Val> {
	type Item = (Key, Val);
	type IntoIter = BTreeIntoIter <Key, Val>;
	fn into_iter (self) -> BTreeIntoIter <Key, Val> {
		self.map.into_iter ()
	}
}

impl <'map, Key, Val> IntoIterator for & 'map HashMap <Key, Val> {
	type Item = (& 'map Key, & 'map Val);
	type IntoIter = BTreeIter <'map, Key, Val>;
	fn into_iter (self) -> BTreeIter <'map, Key, Val> {
		(& self.map).into_iter ()
	}
}

impl <Key, Val> PartialEq for HashMap <Key, Val>
		where Key: PartialEq, Val: PartialEq {
	fn eq (& self, other: & Self) -> bool {
		self.map.eq (& other.map)
	}
}
