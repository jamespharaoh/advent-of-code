use super::*;

pub struct BitHasher <BldHsh: BuildHasher, const LEN: usize, const BITS: usize> {
	data: [u64; LEN],
	hash_builder: BldHsh,
}

impl <BldHsh: BuildHasher, const LEN: usize, const BITS: usize> BitHasher <BldHsh, LEN, BITS> {
	pub fn new_with_hasher (hash_builder: BldHsh) -> BitHasher <BldHsh, LEN, BITS> {
		BitHasher { data: [0; LEN], hash_builder }
	}
	pub fn update <Val: Hash> (& mut self, val: Val) -> & mut Self {
		let mut hasher = self.hash_builder.build_hasher ();
		val.hash (& mut hasher);
		let mut hash = hasher.finish ();
		for _ in 0 .. BITS {
			let bit = 1 << (hash & 0x3f);
			hash >>= 6;
			let idx = (hash % LEN as u64) as usize;
			hash /= LEN as u64;
			self.data [idx] |= bit;
		}
		self
	}
	pub fn finish (& self) -> BitHash <LEN> {
		BitHash { data: self.data }
	}
}

#[ derive (Clone, Copy) ]
pub struct BitHash <const LEN: usize> {
	data: [u64; LEN],
}

impl <const LEN: usize> BitHash <LEN> {
	pub fn zero () -> Self {
		BitHash { data: [0; LEN] }
	}
	pub fn bits (& self) -> usize {
		let mut sum = 0;
		for idx in 0 .. LEN { sum += self.data [idx].count_ones () as usize; }
		sum
	}
	pub fn reduce <const OTHER_LEN: usize> (& self) -> BitHash <OTHER_LEN> {
		if LEN < OTHER_LEN { panic! () }
		let mut data = [0; OTHER_LEN];
		for idx in 0 .. LEN { data [idx % OTHER_LEN] |= self.data [idx]; }
		BitHash { data }
	}
	pub fn is_zero (& self) -> bool {
		for idx in 0 .. LEN { if self.data [idx] != 0 { return false } }
		true
	}
}

impl <const LEN: usize> fmt::Display for BitHash <LEN> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for idx in 0 .. LEN {
			write! (formatter, "{:064b}", self.data [idx]) ?;
		}
		Ok (())
	}
}

impl <const LEN: usize> ops::BitAnd for BitHash <LEN> {
	type Output = BitHash <LEN>;
	fn bitand (mut self, other: Self) -> Self {
		for idx in 0 .. LEN { self.data [idx] &= other.data [idx]; }
		self
	}
}

impl <const LEN: usize> ops::Not for BitHash <LEN> {
	type Output = BitHash <LEN>;
	fn not (mut self) -> Self {
		for idx in 0 .. LEN { self.data [idx] = ! self.data [idx]; }
		self
	}
}

pub trait IteratorBitHash : Iterator {
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
