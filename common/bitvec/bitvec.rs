//! Dynamically sized array of items encoded as bits and packed

use std::cmp;
use std::marker::PhantomData;

use aoc_nums as nums;
use nums::IntConv;

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
		debug_assert! (Encoding::BITS < usize::BITS);
		debug_assert! (Encoding::MASK == (1 << Encoding::BITS) - 1);
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

	/// Add a new item, increases the size by one
	///
	#[ inline ]
	pub fn push (& mut self, item: Item) {
		let item_enc = Encoding::encode (item);
		self.push_real (item_enc);
	}

	fn push_real (& mut self, mut item_enc: usize) {
		debug_assert! (item_enc & ! Encoding::MASK == 0);
		let mut item_bits = Encoding::BITS;
		let mut word_idx = self.len * Encoding::BITS.as_usize () / usize::BITS.as_usize ();
		let mut bit_idx = (self.len * Encoding::BITS.as_usize () % usize::BITS.as_usize ()).as_u32 ();
		while item_bits > 0 {
			let word_bits = cmp::min (item_bits, usize::BITS - bit_idx);
			let word_mask = (1 << word_bits) - 1;
			let mut word_val = if word_idx < self.words.len () {
				self.words [word_idx]
			} else { 0 };
			word_val |= (item_enc & word_mask) << bit_idx;
			if word_idx < self.words.len () {
				self.words [word_idx] = word_val;
			} else {
				self.words.push (word_val);
			}
			item_enc >>= word_bits;
			item_bits -= word_bits;
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

	fn get_real (& self, idx: usize) -> Option <usize> {
		if self.len < idx + 1 { return None }
		let mut item_enc = 0;
		let mut item_bits = 0;
		let mut word_idx = idx * Encoding::BITS.as_usize () / usize::BITS.as_usize ();
		let mut bit_idx = (idx * Encoding::BITS.as_usize () % usize::BITS.as_usize ()).as_u32 ();
		while item_bits < Encoding::BITS {
			let word_bits = cmp::min (Encoding::BITS - item_bits, usize::BITS - bit_idx);
			let word_mask = (1 << word_bits) - 1;
			let word_val = self.words [word_idx];
			item_enc |= ((word_val >> bit_idx) & word_mask) << item_bits;
			item_bits += word_bits;
			word_idx += 1;
			bit_idx = 0;
		}
		Some (item_enc)
	}

	/// Replace the item at the specified index
	///
	#[ inline ]
	pub fn set (& mut self, idx: usize, item: Item) {
		let item_enc = Encoding::encode (item);
		self.set_real (idx, item_enc);
	}

	fn set_real (& mut self, idx: usize, mut item_enc: usize) {
		assert! (idx < self.len, "Tried to set {} but len is {}", idx, self.len);
		let mut item_bits = Encoding::BITS;
		let mut word_idx = idx * Encoding::BITS.as_usize () / usize::BITS.as_usize ();
		let mut bit_idx = (idx * Encoding::BITS.as_usize () % usize::BITS.as_usize ()).as_u32 ();
		while item_bits > 0 {
			let word_bits = cmp::min (item_bits, usize::BITS - bit_idx);
			let word_mask = (1 << word_bits) - 1;
			let mut word_val = self.words [word_idx];
			word_val |= (item_enc & word_mask) << bit_idx;
			self.words [word_idx] = word_val;
			item_enc >>= word_bits;
			item_bits -= word_bits;
			word_idx += 1;
			bit_idx = 0;
		}
	}

	/// Create an iterator over the stored items
	///
	#[ inline ]
	#[ must_use ]
	pub fn iter (& self) -> BitVecIter <Item, Encoding> {
		BitVecIter {
			words: self.words.as_slice (),
			len: self.len,
			word_val: 0,
			word_bits: 0,
			phantom: PhantomData,
		}
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
		for item in iter {
			bitvec.push (item);
		}
		bitvec
	}
}

/// Iterator over the items in a [`BitVec`]
///
#[ derive (Clone) ]
pub struct BitVecIter <'dat, Item, Encoding = BitVecEncodingDefault> {

	/// The remaining packed data
	///
	words: & 'dat [usize],

	/// Number of items remaining
	///
	len: usize,

	/// The word currently being unpacked
	///
	word_val: usize,

	/// Number of bits remaining in the current word
	///
	word_bits: u32,

	/// Phantom data to make the type system happy
	///
	phantom: PhantomData <(Item, Encoding)>,

}

impl <'dat, Item, Encoding> BitVecIter <'dat, Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {

	fn next_real (& mut self) -> Option <usize> {
		if self.len == 0 { return None }
		let mut item_enc = 0;
		let mut item_bits = 0;
		while item_bits < Encoding::BITS {
			let word_bits = cmp::min (Encoding::BITS - item_bits, self.word_bits);
			item_enc |= (self.word_val & ((1 << word_bits) - 1)) << item_bits;
			self.word_val >>= word_bits;
			self.word_bits -= word_bits;
			item_bits += word_bits;
			if self.word_bits == 0 {
				self.word_val = self.words [0];
				self.word_bits = usize::BITS;
				self.words = & self.words [1 .. ];
			}
		}
		self.len -= 1;
		Some (item_enc)
	}

	fn nth_real (& mut self, num: usize) -> Option <usize> {
		let mut adv_bits = Encoding::BITS.as_usize () * num;
		let adv_words =
			(usize::BITS.as_usize () - self.word_bits.as_usize () + adv_bits)
				/ usize::BITS.as_usize ();
		if adv_words > 0 {
			self.word_val = self.words [adv_words - 1];
			self.words = & self.words [adv_words .. ];
			adv_bits -= (adv_words - 1) * usize::BITS.as_usize ();
			self.word_bits = usize::BITS;
		}
		debug_assert! (adv_bits <= self.word_bits.as_usize ());
		self.word_bits -= adv_bits.as_u32 ();
		self.word_val >>= adv_bits;
		self.len -= num;
		self.next_real ()
	}

}

impl <'dat, Item, Encoding> Iterator for BitVecIter <'dat, Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {

	type Item = Item;

	#[ inline ]
	fn next (& mut self) -> Option <Item> {
		self.next_real ().map (Encoding::decode)
	}

	#[ inline ]
	fn nth (& mut self, num: usize) -> Option <Item> {
		self.nth_real (num).map (Encoding::decode)
	}

	#[ inline ]
	fn count (self) -> usize {
		self.len
	}

}

/// Trait for encoding an item as bits for storing in a [`BitVec`]
///
pub trait BitVecEncoding <Item> {

	/// Number of bits in each encoded item
	///
	const BITS: u32;

	/// Mask for the bits in an encoded item
	///
	const MASK: usize = (1 << Self::BITS) - 1;

	/// Encode an item into its representation as bits
	///
	fn encode (item: Item) -> usize;

	/// Decode an item from its representation as bits
	///
	fn decode (bits: usize) -> Item;

}

/// Trait for items which know how to encode themselves for storing in a [`BitVec`]
///
pub trait BitVecNative {

	/// Number of bits in an encoded item
	///
	const BITS: u32;

	/// Encode an item into its representation as bits
	///
	fn encode (self) -> usize;

	/// Decode an item from its representation as bits
	///
	fn decode (encoded: usize) -> Self;

}

/// Default implementation of [`BitVecEncoding`] for items which implement [`BitVecNative`]
///
#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct BitVecEncodingDefault;

impl <Item> BitVecEncoding <Item> for BitVecEncodingDefault where Item: BitVecNative {

	const BITS: u32 = Item::BITS;

	const MASK: usize = (1 << Item::BITS) - 1;

	#[ inline ]
	fn encode (item: Item) -> usize { Item::encode (item) }

	#[ inline ]
	fn decode (bits: usize) -> Item { Item::decode (bits) }

}
