//! Bloom filter

use super::*;
use nums::IntConv;

/// State for building a [`BitHash`] iteratively
///
pub struct BitHasher <BldHsh: BuildHasher, const LEN: usize, const BITS: usize> {

	/// Current hash value
	///
	data: [u64; LEN],

	/// Build hasher state
	///
	hash_builder: BldHsh,

}

impl <BldHsh: BuildHasher, const LEN: usize, const BITS: usize> BitHasher <BldHsh, LEN, BITS> {

	/// Construct a [`BitHasher`] with the provided [`BuildHasher`]
	///
	#[ inline ]
	pub const fn new_with_hasher (hash_builder: BldHsh) -> Self {
		Self { data: [0; LEN], hash_builder }
	}

	/// Update the hash value with a new item
	///
	#[ inline ]
	pub fn update <Val: Hash> (& mut self, val: Val) -> & mut Self {
		let mut hasher = self.hash_builder.build_hasher ();
		val.hash (& mut hasher);
		let mut hash = hasher.finish ();
		for _ in 0 .. BITS {
			let bit = 1 << (hash & 0x3f);
			hash >>= 6_u32;
			let idx = (hash % LEN.to_u64 ().unwrap ()).to_usize ().unwrap ();
			hash /= LEN.to_u64 ().unwrap ();
			self.data [idx] |= bit;
		}
		self
	}

	/// Stop hashing and return the hash value
	///
	#[ inline ]
	pub const fn finish (& self) -> BitHash <LEN> {
		BitHash { data: self.data }
	}

}

/// A bit hash value
///
#[ derive (Clone, Copy) ]
pub struct BitHash <const LEN: usize> {

	/// The hash value
	///
	data: [u64; LEN],

}

impl <const LEN: usize> BitHash <LEN> {

	/// A zero hash value, with no items added
	///
	#[ inline ]
	#[ must_use ]
	pub const fn zero () -> Self {
		Self { data: [0; LEN] }
	}

	/// The number of one bits in the hash value
	///
	#[ inline ]
	#[ must_use ]
	pub fn bits (& self) -> usize {
		let mut sum = 0;
		for idx in 0 .. LEN { sum += self.data [idx].count_ones ().to_usize ().unwrap (); }
		sum
	}

	/// Map the hash to an equivalent hash of a smaller size
	///
	#[ inline ]
	#[ must_use ]
	pub fn reduce <const OTHER_LEN: usize> (& self) -> BitHash <OTHER_LEN> {
		if LEN < OTHER_LEN { panic! () }
		let mut data = [0; OTHER_LEN];
		for idx in 0 .. LEN { data [idx % OTHER_LEN] |= self.data [idx]; }
		BitHash { data }
	}

	/// Return true is this hash has no one bits
	///
	#[ inline ]
	#[ must_use ]
	pub fn is_zero (& self) -> bool {
		for idx in 0 .. LEN { if self.data [idx] != 0 { return false } }
		true
	}

}

impl <const LEN: usize> fmt::Display for BitHash <LEN> {
	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for idx in 0 .. LEN {
			write! (formatter, "{:064b}", self.data [idx]) ?;
		}
		Ok (())
	}
}

impl <const LEN: usize> ops::BitAnd for BitHash <LEN> {

	type Output = Self;

	#[ inline ]
	fn bitand (mut self, other: Self) -> Self {
		for idx in 0 .. LEN { self.data [idx] &= other.data [idx]; }
		self
	}

}

impl <const LEN: usize> ops::Not for BitHash <LEN> {

	type Output = Self;

	#[ inline ]
	fn not (mut self) -> Self {
		for idx in 0 .. LEN { self.data [idx] = ! self.data [idx]; }
		self
	}

}

/// Extension trait to [`Iterator`] for working with [`BitHash`]
///
pub trait IteratorBitHash : Iterator {

	/// Construct a [`BitHash`] with the values from an iterator
	///
	#[ inline ]
	fn bit_hash <BldHsh, const LEN: usize, const BITS: usize> (self, hash_builder: BldHsh) -> BitHash <LEN>
			where BldHsh: BuildHasher, Self: Sized, Self::Item: Hash {
		let mut hasher = BitHasher::<BldHsh, LEN, BITS>::new_with_hasher (hash_builder);
		for item in self { hasher.update (& item); }
		hasher.finish ()
	}

}

impl <SomeIter, SomeItem> IteratorBitHash for SomeIter
	where
		SomeIter: Iterator <Item = SomeItem> + Sized,
		SomeItem: Hash {
}
