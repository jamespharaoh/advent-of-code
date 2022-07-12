use super::*;

#[ derive (Clone, Copy) ]
pub struct BitHash <const LEN: usize> {
	pub data: [u64; LEN],
}

impl <const LEN: usize> BitHash <LEN> {
	pub fn new () -> BitHash <LEN> {
		BitHash { data: [0; LEN] }
	}
	pub fn start <Val: Hash> (val: & Val) -> Self {
		Self::new ().update (val)
	}
	pub fn update <Val: Hash> (self, val: & Val) -> Self {
		let mut hasher = DefaultHasher::new ();
		val.hash (& mut hasher);
		let val = hasher.finish ();
		let idx = (val as usize >> 6) % LEN;
		let mut data = self.data;
		data [idx] |= 1u64 << (val & (u64::BITS as u64 - 1));
		BitHash { data }
	}
	pub fn bits (& self) -> usize {
		self.data.into_iter ().map (|byte| byte.count_ones () as usize).sum ()
	}
	pub fn reduce <const OTHER_LEN: usize> (& self) -> BitHash <OTHER_LEN> {
		if LEN < OTHER_LEN { panic! () }
		let mut data = [0; OTHER_LEN];
		for idx in 0 .. LEN { data [idx % OTHER_LEN] |= self.data [idx]; }
		BitHash { data }
	}
	pub fn is_zero (& self) -> bool {
		self.data.into_iter ().all (|byte| byte == 0)
	}
}

impl <const LEN: usize> fmt::Display for BitHash <LEN> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for byte in self.data.iter ().copied () {
			write! (formatter, "{:064b}", byte) ?;
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
		for byte in self.data.iter_mut () { * byte = ! * byte; }
		self
	}
}
