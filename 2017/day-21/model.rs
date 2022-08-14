use super::*;

pub use squares::TwoByTwo;
pub use squares::ThreeByThree;
pub use squares::FourByFour;
pub use squares::SixBySix;

mod squares {

	use super::*;

	#[ derive (Clone, Copy, Debug, Default, Eq, PartialEq) ]
	pub struct TwoByTwo { val: u8 }

	impl TwoByTwo {

		#[ must_use ]
		pub fn rotate (self) -> Self {
			let mut next = 0;
			for bit in [ 1_u32, 3, 0, 2 ].iter ().copied () {
				next >>= 1_u32;
				if (self.val & (1 << bit)) != 0 { next |= 0x08; }
			}
			Self { val: next }
		}

		#[ must_use ]
		pub fn idx (self) -> usize {
			self.val.as_usize ()
		}

	}

	impl Display for TwoByTwo {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "{:01x}", self.val) ?;
			Ok (())
		}
	}

	impl TryFrom <u8> for TwoByTwo {
		type Error = ();
		fn try_from (val: u8) -> Result <Self, ()> {
			if val > 0xf { return Err (()) }
			Ok (Self { val })
		}
	}

	#[ derive (Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd) ]
	pub struct ThreeByThree { val: u16 }

	impl ThreeByThree {

		#[ must_use ]
		pub fn idx (self) -> usize {
			self.val.as_usize ()
		}

		#[ must_use ]
		pub fn num_active (self) -> u64 {
			self.val.count_ones ().as_u64 ()
		}

		#[ must_use ]
		pub fn rotate (self) -> Self {
			let mut next = 0;
			for bit in [ 2_u32, 5, 8, 1, 4, 7, 0, 3, 6 ].iter ().copied () {
				next >>= 1_u32;
				if (self.val & (1 << bit)) != 0 { next |= 0x100; }
			}
			Self { val: next }
		}

		#[ must_use ]
		pub fn flip (self) -> Self {
			let mut next = 0;
			for bit in [ 2_u32, 1, 0, 5, 4, 3, 8, 7, 6 ].iter ().copied () {
				next >>= 1_u32;
				if (self.val & (1 << bit)) != 0 { next |= 0x100; }
			}
			Self { val: next }

		}

	}

	impl Display for ThreeByThree {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "{:03x}", self.val) ?;
			Ok (())
		}
	}

	impl TryFrom <u16> for ThreeByThree {
		type Error = ();
		fn try_from (val: u16) -> Result <Self, ()> {
			if val > 0x1ff { return Err (()) }
			Ok (Self { val })
		}
	}

	#[ derive (Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd) ]
	pub struct FourByFour { val: u16 }

	impl FourByFour {

		#[ must_use ]
		pub fn idx (self) -> usize {
			self.val.as_usize ()
		}

		#[ must_use ]
		pub fn num_active (self) -> u64 {
			self.val.count_ones ().as_u64 ()
		}

		#[ must_use ]
		pub fn split (self) -> [TwoByTwo; 4] {
			[ [0_u32, 1, 4, 5], [2, 3, 6, 7], [8, 9, 12, 13], [10, 11, 14, 15] ].map (|bits| {
				let mut val = 0;
				for bit in bits.iter ().copied () {
					val >>= 1_u32;
					if self.val & (1 << bit) != 0 { val |= 0x8; }
				}
				TwoByTwo::try_from (val).unwrap ()
			})
		}

	}

	impl Display for FourByFour {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "{:04x}", self.val) ?;
			Ok (())
		}
	}

	impl From <u16> for FourByFour {
		fn from (val: u16) -> Self {
			Self { val }
		}
	}

	#[ derive (Clone, Copy, Debug, Default, Eq, Ord, PartialOrd, PartialEq) ]
	pub struct SixBySix { val: u64 }

	impl SixBySix {

		#[ must_use ]
		pub fn idx (self) -> usize {
			self.val.as_usize ()
		}

		#[ must_use ]
		pub fn num_active (self) -> u64 {
			self.val.count_ones ().as_u64 ()
		}

		#[ must_use ]
		pub fn split (self) -> [TwoByTwo; 9] {
			[
				[0_u32, 1, 6, 7], [2, 3, 8, 9], [4, 5, 10, 11],
				[12, 13, 18, 19], [14, 15, 20, 21], [16, 17, 22, 23],
				[24, 25, 30, 31], [26, 27, 32, 33], [28, 29, 34, 35],
			].map (|bits| {
				let mut val = 0;
				for bit in bits.iter ().copied () {
					val >>= 1_u32;
					if self.val & (1 << bit) != 0 { val |= 0x8; }
				}
				TwoByTwo::try_from (val).unwrap ()
			})
		}

		#[ must_use ]
		pub fn join (squares: [ThreeByThree; 4]) -> Self {
			let mut val = 0;
			for (square_idx, bit) in [
				(0, 0_u32), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2),
				(0, 3), (0, 4), (0, 5), (1, 3), (1, 4), (1, 5),
				(0, 6), (0, 7), (0, 8), (1, 6), (1, 7), (1, 8),
				(2, 0), (2, 1), (2, 2), (3, 0), (3, 1), (3, 2),
				(2, 3), (2, 4), (2, 5), (3, 3), (3, 4), (3, 5),
				(2, 6), (2, 7), (2, 8), (3, 6), (3, 7), (3, 8),
			].iter ().copied () {
				val >>= 1_u32;
				if squares [square_idx].val & (1 << bit) != 0 { val |= 0x8_0000_0000; }
			}
			Self { val }
		}

	}

	impl Display for SixBySix {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "{:09x}", self.val) ?;
			Ok (())
		}
	}

	impl From <u64> for SixBySix {
		fn from (val: u64) -> Self {
			Self { val }
		}
	}

}
