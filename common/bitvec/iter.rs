use super::*;

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

	/// Index of next bit to unpack from current word
	///
	bit_idx: u32,

	/// Phantom data to make the type system happy
	///
	phantom: PhantomData <(Item, Encoding)>,

}

impl <'dat, Item, Encoding> BitVecIter <'dat, Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {

	#[ inline ]
	pub (crate) fn new (words: & 'dat [usize], len: usize) -> Self {
		BitVecIter {
			words: if 0 < len { & words [ 1 .. ] } else { & [] },
			len,
			word_val: if 0 < len { words [0] } else { 0 },
			bit_idx: 0,
			phantom: PhantomData,
		}
	}

	#[ inline ]
	fn next_real (& mut self) -> Option <usize> {
		if self.len == 0 { return None }
		self.len -= 1;
		let mut item_enc = 0;
		let mut rem_bits = Encoding::BITS;
		while 0 < rem_bits {
			item_enc |= Encoding::word_to_item (self.word_val, self.bit_idx);
			if self.bit_idx + rem_bits < usize::BITS {
				self.bit_idx += rem_bits;
				break;
			}
			rem_bits -= usize::BITS - self.bit_idx;
			if 0 < self.len {
				self.word_val = self.words [0];
				self.words = & self.words [1 .. ];
				self.bit_idx = 0;
			} else {
				assert! (rem_bits == 0);
				assert! (self.words.is_empty ());
				self.word_val = 0;
				self.bit_idx = usize::BITS;
			}
		}
		Some (item_enc & Encoding::MASK)
	}

	/*
	fn nth_real (& mut self, num: usize) -> Option <usize> {
		let mut adv_bits = Encoding::BITS.qck_usize () * num;
		let adv_words =
			(usize::BITS.qck_usize () - self.word_bits.qck_usize () + adv_bits)
				/ usize::BITS.qck_usize ();
		if adv_words > 0 {
			self.word_val = self.words [adv_words - 1];
			self.words = & self.words [adv_words .. ];
			adv_bits -= (adv_words - 1) * usize::BITS.qck_usize ();
			self.word_bits = usize::BITS;
		}
		debug_assert! (adv_bits <= self.word_bits.pan_usize ());
		self.word_bits -= adv_bits.qck_u32 ();
		self.word_val >>= adv_bits;
		self.len -= num;
		self.next_real ()
	}
	*/

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

	/*
	#[ inline ]
	fn nth (& mut self, num: usize) -> Option <Item> {
		self.nth_real (num).map (Encoding::decode)
	}
	*/

	#[ inline ]
	fn count (self) -> usize {
		self.len
	}

}
