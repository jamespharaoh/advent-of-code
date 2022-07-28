use super::*;
use nums::IntConv;

#[ derive (Debug, Clone, Eq, PartialEq) ]
pub struct BitVec <Item, Encoding = BitVecEncodingDefault> {
	len: usize,
	words: Vec <usize>,
	phantom: PhantomData <(Item, Encoding)>,
}

impl <Item, Encoding> BitVec <Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {
	pub fn new () -> Self {
		debug_assert! (Encoding::BITS < usize::BITS);
		debug_assert! (Encoding::MASK == (1 << Encoding::BITS) - 1);
		BitVec {
			words: Vec::new (),
			len: 0,
			phantom: PhantomData,
		}
	}
	pub fn len (& self) -> usize { self.len }
	pub fn is_empty (& self) -> bool { self.len == 0 }
	pub fn push (& mut self, item: Item) {
		let mut item_enc = Encoding::encode (item);
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
	pub fn get (& self, idx: usize) -> Option <Item> {
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
		Some (Encoding::decode (item_enc))
	}
	pub fn set (& mut self, idx: usize, item: Item) {
		if self.len < idx + 1 { panic! ("Tried to set {} but len is {}", idx, self.len) }
		let mut item_enc = Encoding::encode (item);
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

impl <Item, Encoding> FromIterator <Item> for BitVec <Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {
	fn from_iter <Iter: IntoIterator <Item = Item>> (iter: Iter) -> BitVec <Item, Encoding> {
		let mut bitvec = BitVec::new ();
		for item in iter.into_iter () {
			bitvec.push (item);
		}
		bitvec
	}
}

#[ derive (Clone) ]
pub struct BitVecIter <'a, Item, Encoding = BitVecEncodingDefault> {
	words: & 'a [usize],
	len: usize,
	word_val: usize,
	word_bits: u32,
	phantom: PhantomData <(Item, Encoding)>,
}

impl <'a, Item, Encoding> Iterator for BitVecIter <'a, Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {
	type Item = Item;
	fn next (& mut self) -> Option <Item> {
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
		Some (Encoding::decode (item_enc))
	}
	fn nth (& mut self, num: usize) -> Option <Item> {
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
		self.next ()
	}
	fn count (self) -> usize {
		self.len
	}
}

pub trait BitVecEncoding <Item> {
	const BITS: u32;
	const MASK: usize = (1 << Self::BITS) - 1;
	fn encode (item: Item) -> usize;
	fn decode (bits: usize) -> Item;
}

pub trait BitVecNative {
	const BITS: u32;
	fn encode (self) -> usize;
	fn decode (encoded: usize) -> Self;
}

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct BitVecEncodingDefault;

impl <Item> BitVecEncoding <Item> for BitVecEncodingDefault where Item: BitVecNative {
	const BITS: u32 = Item::BITS;
	const MASK: usize = (1 << Item::BITS) - 1;
	fn encode (item: Item) -> usize { Item::encode (item) }
	fn decode (bits: usize) -> Item { Item::decode (bits) }
}
