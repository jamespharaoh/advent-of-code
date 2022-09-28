//! Dynamically sized array of items encoded as bits and packed

use std::cmp;
use std::marker::PhantomData;

use aoc_nums as nums;
use nums::IntConv;

mod encode;
mod iter;

pub use encode::*;
pub use iter::*;

pub mod prelude {
	pub use super::BitVec;
	pub use super::BitVecEncoding;
	pub use super::BitVecNative;
}

/// Dynamically sized array of items encoded as bits and packed
///
#[ derive (Debug, Clone, Eq, PartialEq) ]
pub struct BitVec <Item, Encoding = BitVecEncodingDefault> {

	/// The number of items stored
	///
	len: usize,

	/// Data for stored items
	///
	words: Vec <usize>,

	/// Phantom data to make the type system happy
	///
	phantom: PhantomData <(Item, Encoding)>,

}

impl <Item, Encoding> BitVec <Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {

	/// Create a new [`BitVec`] with no items
	///
	#[ inline ]
	#[ must_use ]
	pub fn new () -> Self {
		assert! (Encoding::BITS < usize::BITS);
		assert! (Encoding::MASK == (1 << Encoding::BITS) - 1);
		Self {
			words: Vec::new (),
			len: 0,
			phantom: PhantomData,
		}
	}

	/// Number of items stored
	///
	#[ inline ]
	#[ must_use ]
	pub const fn len (& self) -> usize { self.len }

	/// True if there are no stored items
	///
	#[ inline ]
	#[ must_use ]
	pub const fn is_empty (& self) -> bool { self.len == 0 }

	#[ inline ]
	pub fn extend (& mut self, iter: impl IntoIterator <Item = Item>) {
		let (_, mut bit_idx) = Self::get_backing_indexes (self.len);
		let mut word_val = if 0 < bit_idx { self.words.pop ().unwrap () } else { 0 };
		for item in iter {
			let item_enc = Encoding::encode (item);
			debug_assert! (item_enc & ! Encoding::MASK == 0);
			let mut rem_bits = Encoding::BITS;
			while 0 < rem_bits {
				word_val |= Encoding::item_to_word (item_enc, bit_idx);
				if bit_idx + rem_bits < usize::BITS {
					bit_idx += Encoding::BITS;
					break;
				}
				rem_bits -= usize::BITS - bit_idx;
				self.words.push (word_val);
				word_val = 0;
				bit_idx = 0;
			}
			self.len += 1;
		}
		if 0 < bit_idx { self.words.push (word_val); }
	}

	/// Add a new item, increases the size by one
	///
	#[ inline ]
	pub fn push (& mut self, item: Item) {
		let item_enc = Encoding::encode (item);
		self.push_real (item_enc);
	}

	#[ inline ]
	fn push_real (& mut self, item_enc: usize) {
		debug_assert! (item_enc & ! Encoding::MASK == 0);
		let mut rem_bits = Encoding::BITS;
		let (mut word_idx, mut bit_idx) = Self::get_backing_indexes (self.len);
		while 0 < rem_bits {
			if self.words.len () <= word_idx { self.words.push (0); }
			self.words [word_idx] |= Encoding::item_to_word (item_enc, bit_idx);
			if bit_idx + rem_bits <= usize::BITS { break }
			rem_bits -= usize::BITS - bit_idx;
			word_idx += 1;
			bit_idx = 0;
		}
		self.len += 1;
	}

	/// Get a specific item given its index
	///
	#[ inline ]
	pub fn get (& self, idx: usize) -> Option <Item> {
		self.get_real (idx).map (Encoding::decode)
	}

	#[ inline ]
	fn get_real (& self, idx: usize) -> Option <usize> {
		if self.len < idx + 1 { return None }
		let mut item_enc = 0;
		let mut rem_bits = Encoding::BITS;
		let (mut word_idx, mut bit_idx) = Self::get_backing_indexes (idx);
		while 0 < rem_bits {
			item_enc |= Encoding::word_to_item (self.words [word_idx], bit_idx);
			rem_bits -= cmp::min (rem_bits, usize::BITS - bit_idx);
			word_idx += 1;
			bit_idx = 0;
		}
		Some (item_enc & Encoding::MASK)
	}

	/// Replace the item at the specified index
	///
	#[ inline ]
	pub fn set (& mut self, idx: usize, item: Item) {
		let item_enc = Encoding::encode (item);
		self.set_real (idx, item_enc);
	}

	#[ inline ]
	fn set_real (& mut self, idx: usize, item_enc: usize) {
		assert! (idx < self.len, "Tried to set {} but len is {}", idx, self.len);
		let (mut word_idx, mut bit_idx) = Self::get_backing_indexes (idx);
		let mut rem_bits = Encoding::BITS;
		while 0 < rem_bits {
			self.words [word_idx] &= ! Encoding::item_to_word (Encoding::MASK, bit_idx);
			self.words [word_idx] |= Encoding::item_to_word (item_enc, bit_idx);
			rem_bits -= cmp::min (rem_bits, usize::BITS - bit_idx);
			word_idx += 1;
			bit_idx = 0;
		}
	}

	/// Create an iterator over the stored items
	///
	#[ inline ]
	#[ must_use ]
	pub fn iter (& self) -> BitVecIter <Item, Encoding> {
		BitVecIter::new (self.words.as_slice (), self.len ())
	}

	#[ inline ]
	fn get_backing_indexes (idx: usize) -> (usize, u32) {
		let word_idx = idx * Encoding::BITS.qck_usize () / usize::BITS.qck_usize ();
		let bit_idx = (idx * Encoding::BITS.qck_usize () % usize::BITS.qck_usize ()).qck_u32 ();
		(word_idx, bit_idx)
	}

}

impl <Item, Encoding> Default for BitVec <Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {

	#[ inline ]
	fn default () -> Self {
		Self::new ()
	}

}

impl <Item, Encoding> FromIterator <Item> for BitVec <Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {

	#[ inline ]
	fn from_iter <Iter: IntoIterator <Item = Item>> (iter: Iter) -> Self {
		let mut bitvec = Self::new ();
		//bitvec.extend (iter);
		for item in iter { bitvec.push (item); }
		bitvec
	}

}
