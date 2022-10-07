use super::*;

pub struct PairsMap <Key, Val> {
	keys: MapToIndex <Key>,
	values: Vec <Val>,
}

impl <Key, Val> PairsMap <Key, Val>
	where Key: Clone + Eq + Hash + Ord {

	#[ inline ]
	#[ must_use ]
	pub fn len (& self) -> usize {
		self.keys.len ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn is_empty (& self) -> bool {
		self.keys.is_empty ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn keys (& self) -> & [Key] {
		& self.keys
	}

	#[ inline ]
	#[ must_use ]
	pub fn values (& self) -> & [Val] {
		& self.values
	}

}

impl <Key, Val> FromIterator <(Key, Key, Val)> for PairsMap <Key, Val>
	where Key: Clone + Eq + Hash + Ord, Val: Clone + Default {

	#[ inline ]
	fn from_iter <Iter: IntoIterator <Item = (Key, Key, Val)>> (iter: Iter) -> Self {
		let mut keys = MapToIndex::new ();
		let mut temp = Vec::new ();
		for (key_0, key_1, val) in iter {
			let idx_0 = keys.insert (key_0);
			let idx_1 = keys.insert (key_1);
			temp.push ((idx_0, idx_1, val));
		}
		let mut values: Vec <Val> =
			iter::from_fn (|| Some (Val::default ()))
				.take (keys.len () * keys.len ())
				.collect ();
		for (idx_0, idx_1, val) in temp {
			values [idx_0 * keys.len () + idx_1] = val.clone ();
		}
		Self { keys, values }
	}

}

impl <Key, Val> Index <(usize, usize)> for PairsMap <Key, Val>
	where Key: Clone + Eq + Hash + Ord, Val: Clone + Default {

	type Output = Val;

	#[ inline ]
	fn index (& self, (idx_0, idx_1): (usize, usize)) -> & Val {
		& self.values [idx_0 * self.keys.len () + idx_1]
	}

}
