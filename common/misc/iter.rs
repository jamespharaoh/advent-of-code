use super::prelude::*;

pub trait IteratorExt: Iterator {

	#[ inline ]
	fn all_unique (self) -> bool where Self: Sized, Self::Item: Eq + Hash + Ord {
		let mut items = HashSet::new ();
		for item in self {
			if ! items.insert (item) { return false }
		}
		true
	}

	#[ inline ]
	fn all_equal (mut self) -> bool where Self: Sized, Self::Item: Eq {
		let first = some_or! (self.next (), return true);
		for item in self {
			if item != first { return false }
		}
		true
	}

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
	fn array_combinations <const DIM: usize> (self) -> ArrayCombinations <Self, DIM>
			where Self: Sized, Self::Item: Clone {
		ArrayCombinations::Start {
			inner: self,
		}
	}

	#[ inline ]
	fn array_windows <const LEN: usize> (self) -> ArrayWindows <Self, LEN>
		where
			Self: Sized,
			Self::Item: Clone {
		ArrayWindows {
			inner: self,
			prev: None,
		}
	}

	#[ inline ]
	fn arrays <const LEN: usize> (self) -> Arrays <Self, LEN> where Self: Sized {
		Arrays { inner: self }
	}

	#[ inline ]
	fn cartesian_product <Other> (self, other: Other) -> CartesianProduct <Self, Other>
		where
			Other: Clone + IntoIterator + Sized,
			Self: Sized,
			Self::Item: Clone {
		CartesianProduct::One {
			iter_one: self,
			into_iter_two: other,
		}
	}

	#[ inline ]
	fn circular_array_windows <const LEN: usize> (self) -> CircularArrayWindows <Self, LEN>
		where
			Self: Sized,
			Self::Item: Clone {
		CircularArrayWindows::Start {
			inner: self,
		}
	}

	#[ inline ]
	fn dedup_consecutive (self) -> DedupConsecutive <Self>
		where
			Self: Sized,
			Self::Item: PartialEq {
		DedupConsecutive {
			inner: self,
			last: None,
		}
	}

	#[ inline ]
	fn exactly_one (mut self) -> Option <Self::Item> where Self: Sized {
		let item = self.next () ?;
		if self.next ().is_some () { return None }
		Some (item)
	}

	#[ inline ]
	fn filter_ok <Item, Error, FilterFn> (
		self,
		filter_fn: FilterFn,
	) -> FilterOk <Self, Item, Error, FilterFn>
		where
			FilterFn: FnMut (& Item) -> bool,
			Self: Iterator <Item = Result <Item, Error>> + Sized {
		FilterOk::Inner { inner: self, filter_fn }
	}

	#[ inline ]
	fn flatten_ok <Item, Error> (self) -> FlattenOk <Self, Item, Error>
		where
			Item: IntoIterator,
			Self: Iterator <Item = Result <Item, Error>> + Sized {
		FlattenOk::Inner { inner: self }
	}

	#[ inline ]
	fn fold_ok <FoldFn, In, Out, Error> (
		self,
		mut state: Out,
		mut fold_fn: FoldFn,
	) -> Result <Out, Error>
		where
			FoldFn: FnMut (Out, In) -> Out,
			Self: Iterator <Item = Result <In, Error>> + Sized {
		for item in self {
			match item {
				Ok (item) => state = fold_fn (state, item),
				Err (error) => return Err (error),
			}
		}
		Ok (state)
	}

	#[ inline ]
	fn map_ok <MapFn, In, Out, Error> (self, map_fn: MapFn) -> MapOk <Self, MapFn, In, Out, Error>
		where
			Self: Iterator <Item = Result <In, Error>> + Sized,
			MapFn: FnMut (In) -> Out {
		MapOk {
			inner: self,
			map_fn,
		}
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

	#[ inline ]
	fn merge_consecutive <MergeFn> (self, merge_fn: MergeFn) -> MergeConsecutive <Self, MergeFn>
		where
			MergeFn: FnMut (Self::Item, Self::Item) -> Result <Self::Item, (Self::Item, Self::Item)>,
			Self: Sized {
		MergeConsecutive::Start {
			inner: self,
			merge_fn,
		}
	}

	#[ inline ]
	fn min_ok <Item, Error> (mut self) -> Result <Option <Item>, Error>
		where
			Item: Ord,
			Self: Sized + Iterator <Item = Result <Item, Error>> {
		let mut min = if let Some (item) = self.next () { item ? } else { return Ok (None) };
		for item in self {
			let item = item ?;
			if min <= item { continue }
			min = item;
		}
		Ok (Some (min))
	}

	#[ inline ]
	fn min_max (mut self) -> Option <(Self::Item, Self::Item)>
			where Self: Sized, Self::Item: Clone + Ord {
		let first = some_or! (self.next (), return None);
		let (mut min, mut max) = (first.clone (), first);
		for item in self {
			(min, max) = (cmp::min (min, item.clone ()), cmp::max (max, item));
		}
		Some ((min, max))
	}

	#[ inline ]
	fn multipeek (self) -> MultiPeek <Self>
			where Self: Sized {
		MultiPeek {
			inner: self,
			buffer: VecDeque::new (),
			peek_idx: 0,
		}
	}

	#[ inline ]
	fn sorted (self) -> VecIntoIter <Self::Item>
		where
			Self: Sized,
			Self::Item: Ord {
		let mut vec: Vec <Self::Item> = self.collect ();
		vec.sort ();
		vec.into_iter ()
	}

	#[ inline ]
	fn sorted_by_cached_key <Key, KeyFn> (self, key_fn: KeyFn) -> VecIntoIter <Self::Item>
		where
			Self: Sized,
			Key: Ord,
			KeyFn: FnMut (& Self::Item) -> Key {
		let mut vec: Vec <Self::Item> = self.collect ();
		vec.sort_by_cached_key (key_fn);
		vec.into_iter ()
	}

	#[ inline ]
	fn sorted_by_key <Key, KeyFn> (self, key_fn: KeyFn) -> VecIntoIter <Self::Item>
		where
			Self: Sized,
			Key: Ord,
			KeyFn: FnMut (& Self::Item) -> Key {
		let mut vec: Vec <Self::Item> = self.collect ();
		vec.sort_by_key (key_fn);
		vec.into_iter ()
	}

	#[ inline ]
	fn sorted_unique (self) -> VecIntoIter <Self::Item>
		where
			Self: Sized,
			Self::Item: Ord {
		let mut vec: Vec <Self::Item> = self.collect ();
		vec.sort ();
		vec.dedup ();
		vec.into_iter ()
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
	fn try_collect <In, Out, Error> (self) -> Result <Out, Error>
		where
			Self: Iterator <Item = Result <In, Error>> + Sized,
			Result <Out, Error>: FromIterator <Result <In, Error>> {
		self.collect ()
	}

	#[ inline ]
	fn while_some <Item> (self) -> WhileSome <Self>
			where Self: Sized + Iterator <Item = Option <Item>> {
		WhileSome {
			inner: self,
		}
	}

}

impl <SomeIter: Iterator> IteratorExt for SomeIter {}

pub enum ArrayCombinations <Inner, const LEN: usize>
		where Inner: Iterator {
	Start { inner: Inner },
	Iterating { inners: [Inner; LEN], items: [Inner::Item; LEN] },
	Finished,
	Poison,
}

impl <Inner, const LEN: usize> FusedIterator for ArrayCombinations <Inner, LEN>
	where
		Inner: Clone + Iterator,
		Inner::Item: Clone {
}

impl <Inner, const LEN: usize> Iterator for ArrayCombinations <Inner, LEN>
	where
		Inner: Clone + Iterator,
		Inner::Item: Clone {

	type Item = [Inner::Item; LEN];

	#[ inline ]
	fn next (& mut self) -> Option <Self::Item> {
		match mem::replace (self, Self::Poison) {
			Self::Start { mut inner } => {
				let mut inners = array::from_fn (|_| None);
				let mut items = array::from_fn (|_| None);
				for idx in 0 .. LEN {
					items [idx] = inner.next ();
					if items [idx].is_none () {
						* self = Self::Finished;
						return None;
					}
					inners [idx] = Some (inner.clone ());
				}
				let inners = inners.map (Option::unwrap);
				let items = items.map (Option::unwrap);
				* self = Self::Iterating { inners, items: items.clone () };
				Some (items)
			},
			Self::Iterating { mut inners, mut items } => {
				'OUTER: for idx_0 in (0 .. LEN).rev () {
					if inners [idx_0].size_hint ().1.map_or (false, |num| num < LEN - idx_0) {
						continue;
					}
					if let Some (item) = inners [idx_0].next () {
						items [idx_0] = item;
						for idx_1 in idx_0 + 1 .. LEN {
							inners [idx_1] = inners [idx_1 - 1].clone ();
							if let Some (item) = inners [idx_1].next () {
							items [idx_1] = item;
							} else { continue 'OUTER }
						}
						* self = Self::Iterating { inners, items: items.clone () };
						return Some (items);
					}
				}
				* self = Self::Finished;
				None
			},
			Self::Finished => None,
			Self::Poison => panic! ("Poisoned"),
		}
	}

}

pub struct ArrayWindows <Inner, const LEN: usize>
		where Inner: Iterator {
	inner: Inner,
	prev: Option <[Inner::Item; LEN]>,
}

impl <Inner, const LEN: usize> FusedIterator for ArrayWindows <Inner, LEN>
	where
		Inner: Iterator,
		Inner::Item: Clone {
}

impl <Inner, const LEN: usize> Iterator for ArrayWindows <Inner, LEN>
	where
		Inner: Iterator,
		Inner::Item: Clone {

	type Item = [Inner::Item; LEN];

	#[ inline ]
	fn next (& mut self) -> Option <Self::Item> {
		if let Some (prev) = self.prev.take () {
			self.inner.next ().map (|item| {
				let array = array::from_fn (|idx|
					if idx == LEN - 1 { item.clone () } else { prev [idx + 1].clone () });
				self.prev = Some (array.clone ());
				array
			})
		} else {
			let mut array = array::from_fn (|_| None);
			for idx in 0 .. LEN {
				array [idx] = self.inner.next ();
				array [idx].as_ref () ?;
			}
			let array = array.map (Option::unwrap);
			self.prev = Some (array.clone ());
			Some (array)
		}
	}

}

pub struct Arrays <Inner, const LEN: usize> where Inner: Iterator {
	inner: Inner,
}

impl <Inner, const LEN: usize> FusedIterator for Arrays <Inner, LEN>
	where Inner: Iterator {
}

impl <Inner, const LEN: usize> Iterator for Arrays <Inner, LEN>
	where Inner: Iterator {

	type Item = [Inner::Item; LEN];

	#[ inline ]
	fn next (& mut self) -> Option <Self::Item> {
		let mut array = array::from_fn (|_| None);
		for idx in 0 .. LEN {
			array [idx] = self.inner.next ();
			array [idx].as_ref () ?;
		}
		Some (array.map (Option::unwrap))
	}

}

pub enum CartesianProduct <IterOne, IntoIterTwo>
	where
		IterOne: Iterator,
		IterOne::Item: Clone,
		IntoIterTwo: Clone + IntoIterator {
	One {
		iter_one: IterOne,
		into_iter_two: IntoIterTwo,
	},
	Two {
		iter_one: IterOne,
		into_iter_two: IntoIterTwo,
		item_one: IterOne::Item,
		iter_two: IntoIterTwo::IntoIter,
	},
	Finished,
	Poison,
}

impl <IterOne, IntoIterTwo> FusedIterator for CartesianProduct <IterOne, IntoIterTwo>
	where
		IterOne: Iterator,
		IterOne::Item: Clone,
		IntoIterTwo: Clone + IntoIterator {
}

impl <IterOne, IntoIterTwo> Iterator for CartesianProduct <IterOne, IntoIterTwo>
	where
		IterOne: Iterator,
		IterOne::Item: Clone,
		IntoIterTwo: Clone + IntoIterator {

	type Item = (IterOne::Item, IntoIterTwo::Item);

	#[ inline ]
	fn next (& mut self) -> Option <Self::Item> {
		loop {
			match mem::replace (self, Self::Poison) {
				Self::One { mut iter_one, into_iter_two } => {
					if let Some (item_one) = iter_one.next () {
						let iter_two = into_iter_two.clone ().into_iter ();
						* self = Self::Two { iter_one, into_iter_two, item_one, iter_two };
					} else {
						* self = Self::Finished;
					}
				},
				Self::Two { iter_one, into_iter_two, item_one, mut iter_two } => {
					if let Some (item_two) = iter_two.next () {
						let item_one_cloned = item_one.clone ();
						* self = Self::Two { iter_one, into_iter_two, item_one, iter_two };
						return Some ((item_one_cloned, item_two));
					}
					* self = Self::One { iter_one, into_iter_two };
				},
				Self::Finished => return None,
				Self::Poison => panic! ("Poisoned"),
			}
		}
	}

}

pub enum CircularArrayWindows <Inner, const LEN: usize>
		where Inner: Iterator {
	Start {
		inner: Inner,
	},
	Main {
		inner: Inner,
		first: [Inner::Item; LEN],
		prev: [Inner::Item; LEN],
	},
	Wrap {
		first: [Inner::Item; LEN],
		prev: [Inner::Item; LEN],
		idx: usize,
	},
	Finished,
	Poison,
}

impl <Inner, const LEN: usize> FusedIterator for CircularArrayWindows <Inner, LEN>
	where
		Inner: Iterator,
		Inner::Item: Clone {
}

impl <Inner, const LEN: usize> Iterator for CircularArrayWindows <Inner, LEN>
	where
		Inner: Iterator,
		Inner::Item: Clone {

	type Item = [Inner::Item; LEN];

	#[ inline ]
	fn next (& mut self) -> Option <Self::Item> {
		loop {
			match mem::replace (self, Self::Poison) {
				Self::Start { mut inner } => {
					let mut array = array::from_fn (|_| None);
					for idx in 0 .. LEN {
						array [idx] = inner.next ();
						if array [idx].is_none () {
							* self = Self::Finished;
							continue;
						}
					}
					let array = array.map (Option::unwrap);
					* self = Self::Main { inner, first: array.clone (), prev: array.clone () };
					return Some (array);
				},
				Self::Main { mut inner, first, prev } => {
					if let Some (item) = inner.next () {
						let array = array::from_fn (|idx|
							if idx == LEN - 1 { item.clone () } else { prev [idx + 1].clone () });
						* self = Self::Main { inner, first, prev: array.clone () };
						return Some (array);
					}
					* self = Self::Wrap { first, prev, idx: 0 }
				},
				Self::Wrap { first, prev, mut idx } => {
					idx += 1;
					if LEN <= idx {
						* self = Self::Finished;
						continue;
					}
					let array = array::from_fn (|arr_idx| {
						if arr_idx + 1 < LEN {
							prev [arr_idx + idx].clone ()
						} else {
							first [arr_idx + idx - LEN].clone ()
						}
					});
					* self = Self::Wrap { first, prev, idx };
					return Some (array);
				},
				Self::Finished => return None,
				Self::Poison => panic! ("Poisoned"),
			}
		}
	}

}

pub struct DedupConsecutive <Inner>
	where
		Inner: Iterator,
		Inner::Item: PartialEq {
	inner: Inner,
	last: Option <Inner::Item>,
}

impl <Inner> FusedIterator for DedupConsecutive <Inner>
	where
		Inner: Iterator,
		Inner::Item: Clone + PartialEq {
}

impl <Inner> Iterator for DedupConsecutive <Inner>
	where
		Inner: Iterator,
		Inner::Item: Clone + PartialEq {

	type Item = Inner::Item;

	#[ inline ]
	fn next (& mut self) -> Option <Self::Item> {
		for item in self.inner.by_ref () {
			if self.last.as_ref () == Some (& item) { continue }
			self.last = Some (item.clone ());
			return Some (item);
		}
		None
	}

}

pub enum FilterOk <Inner, Item, Error, FilterFn>
	where
		FilterFn: FnMut (& Item) -> bool,
		Inner: Iterator <Item = Result <Item, Error>> {
	Inner { inner: Inner, filter_fn: FilterFn },
	Finished,
	Poison,
}

impl <Inner, Item, Error, FilterFn> FusedIterator for FilterOk <Inner, Item, Error, FilterFn>
	where
		FilterFn: FnMut (& Item) -> bool,
		Inner: Iterator <Item = Result <Item, Error>> {
}

impl <Inner, Item, Error, FilterFn> Iterator for FilterOk <Inner, Item, Error, FilterFn>
	where
		FilterFn: FnMut (& Item) -> bool,
		Inner: Iterator <Item = Result <Item, Error>> {

	type Item = Result <Item, Error>;

	#[ inline ]
	fn next (& mut self) -> Option <Self::Item> {
		loop {
			match mem::replace (self, Self::Poison) {
				Self::Inner { mut inner, mut filter_fn } => match inner.next () {
					Some (Ok (item)) => {
						if filter_fn (& item) {
							* self = Self::Inner { inner, filter_fn };
							return Some (Ok (item));
						} else {
							* self = Self::Inner { inner, filter_fn };
						}
					},
					Some (Err (error)) => {
						* self = Self::Inner { inner, filter_fn };
						return Some (Err (error));
					},
					None => {
						* self = Self::Finished;
					},
				},
				Self::Finished => return None,
				Self::Poison => panic! ("Poisoned"),
			}
		}
	}

}

pub enum FlattenOk <Inner, Item, Error>
	where
		Inner: Iterator <Item = Result <Item, Error>>,
		Item: IntoIterator {
	Inner { inner: Inner },
	Nested { inner: Inner, nested: Item::IntoIter },
	Finished,
	Poison,
}

impl <Inner, Item, Error> FusedIterator for FlattenOk <Inner, Item, Error>
	where
		Inner: Iterator <Item = Result <Item, Error>>,
		Item: IntoIterator {
}

impl <Inner, Item, Error> Iterator for FlattenOk <Inner, Item, Error>
	where
		Inner: Iterator <Item = Result <Item, Error>>,
		Item: IntoIterator {

	type Item = Result <Item::Item, Error>;

	#[ inline ]
	fn next (& mut self) -> Option <Self::Item> {
		loop {
			match mem::replace (self, Self::Poison) {
				Self::Inner { mut inner } => match inner.next () {
					Some (Ok (nested)) => {
						* self = Self::Nested { inner, nested: nested.into_iter () };
					},
					Some (Err (error)) => {
						* self = Self::Finished;
						return Some (Err (error));
					},
					None => {
						* self = Self::Finished;
					},
				},
				Self::Nested { inner, mut nested } => match nested.next () {
					Some (item) => {
						* self = Self::Nested { inner, nested };
						return Some (Ok (item));
					},
					None => {
						* self = Self::Inner { inner };
					},
				},
				Self::Finished => return None,
				Self::Poison => panic! ("Poisoned"),
			}
		}
	}

}

pub struct MapOk <Inner, MapFn, In, Out, Error>
	where
		Inner: Iterator <Item = Result <In, Error>>,
		MapFn: FnMut (In) -> Out {
	inner: Inner,
	map_fn: MapFn,
}

impl <Inner, MapFn, In, Out, Error> FusedIterator for MapOk <Inner, MapFn, In, Out, Error>
	where
		Inner: Iterator <Item = Result <In, Error>>,
		MapFn: FnMut (In) -> Out {
}

impl <Inner, MapFn, In, Out, Error> Iterator for MapOk <Inner, MapFn, In, Out, Error>
	where
		Inner: Iterator <Item = Result <In, Error>>,
		MapFn: FnMut (In) -> Out {

	type Item = Result <Out, Error>;

	#[ inline ]
	fn next (& mut self) -> Option <Result <Out, Error>> {
		match self.inner.next () {
			Some (Ok (item)) => Some (Ok ((self.map_fn) (item))),
			Some (Err (error)) => Some (Err (error)),
			None => None,
		}
	}

}

pub enum MergeConsecutive <Inner, MergeFn>
	where
		Inner: Iterator,
		MergeFn: FnMut (Inner::Item, Inner::Item) -> Result <Inner::Item, (Inner::Item, Inner::Item)> {
	Start { inner: Inner, merge_fn: MergeFn },
	Prev { inner: Inner, merge_fn: MergeFn, prev: Inner::Item },
	Finished,
	Poison,
}

impl <Inner, MergeFn> FusedIterator for MergeConsecutive <Inner, MergeFn>
	where
		Inner: Iterator,
		MergeFn: FnMut (Inner::Item, Inner::Item) -> Result <Inner::Item, (Inner::Item, Inner::Item)> {
}

impl <Inner, MergeFn> Iterator for MergeConsecutive <Inner, MergeFn>
	where
		Inner: Iterator,
		MergeFn: FnMut (Inner::Item, Inner::Item) -> Result <Inner::Item, (Inner::Item, Inner::Item)> {

	type Item = Inner::Item;

	#[ inline ]
	fn next (& mut self) -> Option <Inner::Item> {
		loop {
			match mem::replace (self, Self::Poison) {
				Self::Start { mut inner, merge_fn } => match inner.next () {
					Some (prev) => {
						* self = Self::Prev { inner, merge_fn, prev };
					},
					None => {
						* self = Self::Finished;
					},
				},
				Self::Prev { mut inner, mut merge_fn, prev } => {
					if let Some (next) = inner.next () {
						match merge_fn (prev, next) {
							Ok (prev) => {
								* self = Self::Prev { inner, merge_fn, prev };
							},
							Err ((item, prev)) => {
								* self = Self::Prev { inner, merge_fn, prev };
								return Some (item);
							},
						}
					} else {
						* self = Self::Finished;
						return Some (prev);
					}
				},
				Self::Finished => return None,
				Self::Poison => panic! ("Poisoned"),
			}
		}
	}

}

pub struct MultiPeek <Inner> where Inner: Iterator {
	inner: Inner,
	buffer: VecDeque <Inner::Item>,
	peek_idx: usize,
}

impl <Inner> MultiPeek <Inner> where Inner: Iterator {

	#[ inline ]
	pub fn peek (& mut self) -> Option <& Inner::Item> {
		if self.buffer.len () <= self.peek_idx {
			if let Some (item) = self.inner.next () {
				self.buffer.push_back (item);
			}
		}
		let item = self.buffer.get (self.peek_idx);
		if item.is_some () { self.peek_idx += 1; }
		item
	}

	#[ inline ]
	pub fn reset_peek (& mut self) {
		self.peek_idx = 0;
	}

}

impl <Inner> Iterator for MultiPeek <Inner> where Inner: Iterator {

	type Item = Inner::Item;

	#[ inline ]
	fn next (& mut self) -> Option <Inner::Item> {
		self.peek_idx = 0;
		if let Some (item) = self.buffer.pop_front () { return Some (item) }
		self.inner.next ()
	}

}

pub struct WhileSome <Inner> {
	inner: Inner,
}

impl <Inner, Item> FusedIterator for WhileSome <Inner>
	where Inner: FusedIterator <Item = Option <Item>> {
}

impl <Inner, Item> Iterator for WhileSome <Inner>
	where Inner: Iterator <Item = Option <Item>> {

	type Item = Item;

	#[ inline ]
	fn next (& mut self) -> Option <Item> {
		self.inner.next ().flatten ()
	}

}
