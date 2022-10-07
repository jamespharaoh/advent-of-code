#[ cfg (doc) ]
use super::*;

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

	#[ inline ]
	#[ must_use ]
	fn get_bit_shift (bit_idx: u32) -> u32 {
		usize::BITS - bit_idx - Self::BITS
	}

	#[ inline ]
	#[ must_use ]
	fn item_to_word (item_enc: usize, bit_idx: u32) -> usize {
		item_enc << Self::get_bit_shift (bit_idx)
	}

	#[ inline ]
	#[ must_use ]
	fn word_to_item (word_val: usize, bit_idx: u32) -> usize {
		word_val >> Self::get_bit_shift (bit_idx)
	}

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

impl BitVecNative for bool {

	const BITS: u32 = 1;

	#[ inline ]
	fn encode (self) -> usize {
		usize::from (self)
	}

	#[ inline ]
	fn decode (encoded: usize) -> Self {
		encoded != 0
	}

}
