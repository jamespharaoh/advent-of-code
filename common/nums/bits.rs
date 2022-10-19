use super::*;

pub struct BitPusher <Data: Int> {
	data: Data,
	bits: u32,
}

impl <Data: Int> BitPusher <Data> {

	#[ inline (always) ]
	#[ must_use ]
	pub fn new () -> Self {
		Self { data: Data::ZERO, bits: 0 }
	}

	#[ inline (always) ]
	pub fn push <Val: Int + QuickInto <Data>> (& mut self, val: Val, bits: u32) {
		debug_assert! (self.bits + bits <= Data::BITS);
		self.data <<= bits;
		self.data |= val.quick_into ();
		self.bits += bits;
	}

	#[ inline (always) ]
	pub fn finish (self) -> Data {
		self.data << (Data::BITS - self.bits)
	}

}

impl <Data: Int> Default for BitPusher <Data> {

	#[ inline (always) ]
	fn default () -> Self {
		Self::new ()
	}

}

pub struct BitPopper <Data: Int> {
	data: Data,
	bits: u32,
}

impl <Data: Int> BitPopper <Data> {

	#[ inline (always) ]
	pub fn new (data: Data) -> Self {
		Self { data, bits: Data::BITS }
	}

	#[ inline (always) ]
	pub fn pop <Val: Int + QuickFrom <Data>> (& mut self, bits: u32) -> Val {
		debug_assert! (bits <= Val::BITS);
		debug_assert! (bits <= self.bits);
		let result = (self.data >> (Data::BITS - bits)).quick_into ();
		self.data <<= bits;
		self.bits -= bits;
		result
	}

}
