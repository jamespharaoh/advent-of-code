use super::*;

pub type TwoByTwo = Square <u8, 2>;
pub type ThreeByThree = Square <u16, 3>;
pub type FourByFour = Square <u16, 4>;
pub type SixBySix = Square <u64, 6>;

#[ derive (Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd) ]
pub struct Square <Val: Int, const SIZE: u32> {
	val: Val,
}

impl <Val: Int, const SIZE: u32> Square <Val, SIZE> {

	#[ inline ]
	pub fn idx (self) -> usize {
		self.val.pan_usize ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn num_active (self) -> u64 {
		self.val.gen_count_ones ().pan_u64 ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn rotate (self) -> Self where Val: From <bool> {
		let mut val = Val::ZERO;
		for col in 0 .. SIZE {
			for row in (0 .. SIZE).rev () {
				val = (val << 1) | Val::from (self.val.check_bit (row * SIZE + col));
			}
		}
		assert! (val.pan_u32 () < 1 << (SIZE * SIZE));
		Self { val }
	}

	#[ inline ]
	#[ must_use ]
	pub fn flip (self) -> Self where Val: From <bool> {
		let mut val = Val::ZERO;
		for row in (0 .. SIZE).rev () {
			for col in 0 .. SIZE {
				val = (val << 1) | Val::from (self.val.check_bit (row * SIZE + col));
			}
		}
		assert! (val.pan_u32 () < 1 << (SIZE * SIZE));
		Self { val }
	}

}

impl Square <u16, 4> {

	#[ inline ]
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

impl Square <u64, 6> {

	#[ inline ]
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

	#[ inline ]
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

impl <Val: Int, const SIZE: u32> Display for Square <Val, SIZE> {
	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "{}", self.val)
	}
}

impl <const SIZE: u32> TryFrom <u8> for Square <u8, SIZE> {
	type Error = ();
	#[ inline ]
	fn try_from (val: u8) -> Result <Self, ()> {
		if 1 << (SIZE * SIZE) <= val.pan_u32 () { return Err (()) }
		Ok (Self { val })
	}
}

impl <const SIZE: u32> TryFrom <u16> for Square <u16, SIZE> {
	type Error = ();
	#[ inline ]
	fn try_from (val: u16) -> Result <Self, ()> {
		if 1 << (SIZE * SIZE) <= val.pan_u32 () { return Err (()) }
		Ok (Self { val })
	}
}
