//! Implementation of lisp-like lists
//!
//! A list is represented as the first list item and a reference to another list to represent the
//! remaining items - these are referred to as the "head" and the "tail". There is a special case
//! for the empty list.
//!
//! While this is much less efficient than a [`Vec`], it allows reuse in a way which can be very
//! beneficial for certain types of algorithm. Adding new items to a list only creates one new
//! allocation per item, and removing any number of items from the start of the list requires no
//! allocation.
//!
//! We also declare [`CharList`] as `List <char>`, and implement some extra methods and traits to
//! make it work nicely with [`String`] and [`prim@str`].

use aoc_misc::prelude::*;

pub use base_list::List;
pub use char_list::CharList;

mod base_list {

	use super::*;

	pub enum List <Item> {
		Empty,
		Present (Rc <(Item, List <Item>)>),
	}

	impl <Item: Clone> List <Item> {

		#[ inline ]
		#[ must_use ]
		pub const fn new () -> Self { Self::Empty }

		#[ inline ]
		#[ must_use ]
		pub fn cons (& self) -> Option <(& Item, & Self)> {
			match * self {
				Self::Present (ref inner) => {
					let (ref head, ref tail) = * inner.as_ref ();
					Some ((head, tail))
				},
				Self::Empty => None,
			}
		}

		#[ inline ]
		#[ must_use ]
		pub fn head (& self) -> Option <& Item> {
			self.cons ().map (|(head, _)| head)
		}

		#[ inline ]
		#[ must_use ]
		pub fn tail (& self) -> Option <& Self> {
			self.cons ().map (|(_, tail)| tail)
		}

		#[ inline ]
		#[ must_use ]
		pub fn is_empty (& self) -> bool {
			self.cons ().is_none ()
		}

		#[ inline ]
		#[ must_use ]
		pub fn len (& self) -> usize {
			let mut cur = self.clone ();
			let mut len = 0;
			while let Self::Present (inner) = cur {
				let (_, ref new) = * inner.as_ref ();
				cur = new.clone ();
				len += 1;
			}
			len
		}

		#[ inline ]
		#[ must_use ]
		pub fn with_push_front (& self, head: Item) -> Self {
			Self::Present (Rc::new ((head, self.clone ())))
		}

		#[ inline ]
		#[ must_use ]
		pub const fn iter (& self) -> ListIter <Item> {
			ListIter { list: self }
		}

		#[ inline ]
		pub fn contains (& self, item: & Item) -> bool
				where Item: Eq {
			let mut cur = self.clone ();
			while let Self::Present (ref inner) = cur {
				let (ref head, ref tail) = * inner.as_ref ();
				if head == item { return true }
				cur = tail.clone ();
			}
			false
		}

	}

	impl <Item: Clone + Debug> Debug for List <Item> {

		#[ inline ]
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "[") ?;
			let mut cur = self;
			let mut idx = 0_usize;
			while let Some ((head, tail)) = cur.cons () {
				if idx > 0 { write! (formatter, ", ") ?; }
				Debug::fmt (& head, formatter) ?;
				cur = tail;
				idx += 1;
			}
			write! (formatter, "]") ?;
			Ok (())
		}

	}

	impl <Item: Clone> Clone for List <Item> {

		#[ inline ]
		fn clone (& self) -> Self {
			match * self {
				Self::Present (ref inner) => Self::Present (Rc::clone (inner)),
				Self::Empty => Self::Empty,
			}
		}

	}

	impl <Item: Clone + PartialEq> PartialEq for List <Item> {

		#[ inline ]
		fn eq (& self, other: & Self) -> bool {
			let mut left = self;
			let mut right = other;
			loop {
				match (left.cons (), right.cons ()) {
					(None, None) => return true,
					(Some (_), None) | (None, Some (_)) => return false,
					(Some ((left_head, left_tail)), Some ((right_head, right_tail))) => {
						if ! Item::eq (left_head, right_head) { return false }
						(left, right) = (left_tail, right_tail);
					},
				}
			}
		}

	}

	impl <Item> PartialOrd for List <Item> where Item: Clone + PartialOrd {
		#[ inline ]
		fn partial_cmp (& self, other: & Self) -> Option <Ordering> {
			let mut left = self;
			let mut right = other;
			loop {
				match (left.cons (), right.cons ()) {
					(None, None) => return Some (Ordering::Equal),
					(None, Some (_)) => return Some (Ordering::Less),
					(Some (_), None) => return Some (Ordering::Greater),
					(Some ((left_head, left_tail)), Some ((right_head, right_tail))) => {
						match left_head.partial_cmp (right_head) {
							Some (Ordering::Equal) => {
								left = left_tail;
								right = right_tail;
							},
							Some (Ordering::Greater) => return Some (Ordering::Greater),
							Some (Ordering::Less) => return Some (Ordering::Less),
							None => return None,
						}
					},
				}
			}
		}
	}

	impl <Item> Ord for List <Item> where Item: Clone + Ord {
		#[ inline ]
		fn cmp (& self, other: & Self) -> Ordering {
			let mut left = self;
			let mut right = other;
			loop {
				match (left.cons (), right.cons ()) {
					(None, None) => return Ordering::Equal,
					(None, Some (_)) => return Ordering::Less,
					(Some (_), None) => return Ordering::Greater,
					(Some ((left_head, left_tail)), Some ((right_head, right_tail))) => {
						match left_head.cmp (right_head) {
							Ordering::Equal => {
								left = left_tail;
								right = right_tail;
							},
							Ordering::Greater => return Ordering::Greater,
							Ordering::Less => return Ordering::Less,
						}
					},
				}
			}
		}
	}

	impl <Item: Clone + Eq> Eq for List <Item> { }

	impl <Item: Clone + Hash> Hash for List <Item> {

		#[ inline ]
		fn hash <Hshr: Hasher> (& self, hasher: & mut Hshr) {
			let mut cur = self;
			while let Some ((head, tail)) = cur.cons () {
				head.hash (hasher);
				cur = tail;
			}
		}

	}

	impl Display for List <char> {

		#[ inline ]
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			let mut cur = self.clone ();
			while let Self::Present (inner) = cur {
				let & (head, ref tail) = inner.as_ref ();
				write! (formatter, "{}", head) ?;
				cur = tail.clone ();
			}
			Ok (())
		}

	}

	#[ derive (Clone) ]
	pub struct ListIter <'dat, Item: Clone> {
		list: & 'dat List <Item>,
	}

	impl <'dat, Item: Clone> Iterator for ListIter <'dat, Item> {
		type Item = & 'dat Item;
		fn next (& mut self) -> Option <& 'dat Item> {
			self.list.cons ().map (|(head, tail)| {
				self.list = tail;
				head
			})
		}
	}

}

mod char_list {

	use super::*;

	pub type CharList = List <char>;

	impl CharList {

		#[ inline ]
		#[ must_use ]
		pub fn starts_with (& self, pat: & str) -> bool {
			self.strip_prefix (pat).is_some ()
		}

		#[ inline ]
		#[ must_use ]
		pub fn strip_prefix (& self, pat: & str) -> Option <& Self> {
			let mut cur = self;
			let mut pat_chars = pat.chars ();
			loop {
				match (cur.cons (), pat_chars.next ()) {
					(_, None) => return Some (cur),
					(None, _) => return None,
					(Some ((& head, tail)), Some (pat_ch)) => {
						if head != pat_ch { return None }
						cur = tail;
					},
				}
			}
		}

		#[ inline ]
		#[ must_use ]
		pub fn prepend (& self, prefix: & str) -> Self {
			let mut cur = self.clone ();
			for prefix_ch in prefix.chars ().rev () {
				cur = cur.with_push_front (prefix_ch);
			}
			cur
		}

	}

	impl From <& str> for CharList {
		#[ inline ]
		fn from (src: & str) -> Self {
			Self::Empty.prepend (src)
		}
	}

	impl From <& String> for CharList {
		#[ inline ]
		fn from (src: & String) -> Self {
			Self::Empty.prepend (src)
		}
	}

	impl PartialEq <str> for CharList {
		#[ inline ]
		fn eq (& self, other: & str) -> bool {
			let mut cur = self;
			let mut other_iter = other.chars ();
			loop {
				match (cur.cons (), other_iter.next ()) {
					(Some (_), None) | (None, Some (_)) => return false,
					(None, None) => return true,
					(Some ((& cur_head, cur_tail)), Some (other_ch)) => {
						if cur_head != other_ch { return false }
						cur = cur_tail;
					},
				}
			}
		}
	}

	impl PartialEq <String> for CharList {
		#[ inline ]
		fn eq (& self, other: & String) -> bool {
			PartialEq::eq (self, other.as_str ())
		}
	}

	impl PartialEq <CharList> for & String {
		#[ inline ]
		fn eq (& self, other: & CharList) -> bool {
			PartialEq::eq (self.as_str (), other)
		}
	}

	impl PartialEq <CharList> for str {
		#[ inline ]
		fn eq (& self, other: & CharList) -> bool {
			PartialEq::eq (other, self)
		}
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;
		use CharList as CL;

		fn cl (src: & str) -> CL { CL::from (src) }

		const SAMPLES: & [& str] = & [ "", "a", "ab", "abc", "b", "bc", "c" ];

		#[ test ]
		fn eq () {
			for left in SAMPLES.iter ().cloned () {
				for right in SAMPLES.iter ().cloned () {
					if left == right {
						assert! (cl (left) == cl (right),
							"{:?} == {:?} but CharList::from ({:?}) != CharList::from ({:?})",
							left, right, left, right);
					} else {
						assert! (cl (left) != cl (right),
							"{:?} != {:?} but CharList::from ({:?}) == CharList::from ({:?})",
							left, right, left, right);
					}
				}
			}
		}

		#[ test ]
		fn strip_prefix () {
			for left in SAMPLES.iter ().cloned () {
				for right in SAMPLES.iter ().cloned () {
					let expected = left.strip_prefix (right);
					let actual = cl (left).strip_prefix (right).cloned ();
					assert_eq! (expected.map (cl), actual,
						"{:?}.strip_prefix ({:?}) == {:?} but {:?}.strip_prefix ({:?}) == {:?}",
						left, right, expected, cl (left), right, actual);
				}
			}
		}

	}

}
