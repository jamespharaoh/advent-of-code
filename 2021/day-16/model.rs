use super::*;

pub use bit_iter::BitIter;
pub use packet::Packet;

mod bit_iter {

	use super::*;

	pub struct BitIter <'dat> {
		data: & 'dat [u8],
		val: u8,
		val_bits: u8,
		position: usize,
	}

	impl <'dat> BitIter <'dat> {

		#[ inline ]
		#[ must_use ]
		pub const fn new (data: & 'dat [u8]) -> BitIter <'dat> {
			BitIter {
				data,
				val: 0,
				val_bits: 0,
				position: 0,
			}
		}

		#[ inline ]
		#[ must_use ]
		pub const fn position (& self) -> usize {
			self.position
		}

		#[ inline ]
		pub fn next_uint (& mut self, bits: u8) -> Option <u64> {
			assert! (bits <= 64);
			let mut val = 0;
			for _ in 0 .. bits {
				let next_bit = some_or! (self.next_bit (), return None);
				val = val << 1_i32 | u64::from (next_bit);
			}
			Some (val)
		}

		#[ inline ]
		pub fn next_bit (& mut self) -> Option <bool> {
			if ! self.has_next () { return None }
			self.position += 1;
			if self.val_bits == 0 {
				self.val = self.data [0];
				self.val_bits = 8;
				self.data = & self.data [1 .. ];
			}
			let result = self.val & 0x80 != 0;
			self.val <<= 1_u32;
			self.val_bits -= 1;
			Some (result)
		}

		#[ inline ]
		pub fn has_next (& mut self) -> bool {
			0 < self.val_bits || ! self.data.is_empty ()
		}

	}

}

mod packet {

	use super::*;

	#[ allow (dead_code) ]
	#[ derive (Debug) ]
	pub struct Packet {
		pub version: u64,
		pub packet_type: u64,
		pub value: u64,
		pub children: Vec <Packet>,
	}

	impl Packet {

		#[ allow (clippy::missing_inline_in_public_items) ]
		pub fn decode (iter: & mut BitIter <'_>) -> Option <Self> {
			if ! iter.has_next () { return None }
			let version = iter.next_uint (3) ?;
			let packet_type = iter.next_uint (3) ?;
			let mut value = 0;
			let mut children = Vec::new ();
			if packet_type == 4 {
				let mut bits = 0_i32;
				loop {
					let is_last = ! iter.next_bit () ?;
					if bits + 4_i32 > 64_i32 { return None }
					value = value << 4_i32 | iter.next_uint (4) ?;
					bits += 4_i32;
					if is_last { break }
				}
			} else if ! iter.next_bit () ? {
				let child_bits = iter.next_uint (15) ?;
				let end_position = iter.position () + child_bits.pan_usize ();
				while iter.position () < end_position {
					children.push (Self::decode (iter) ?);
				}
			} else {
				let num_children = iter.next_uint (11) ?;
				for _ in 0 .. num_children {
					children.push (Self::decode (iter) ?);
				}
			}
			Some (Self { version, packet_type, value, children })
		}

		#[ allow (clippy::missing_inline_in_public_items) ]
		#[ must_use ]
		pub fn version_sum (& self) -> u64 {
			self.version + self.children.iter ()
				.map (Self::version_sum)
				.sum::<u64> ()
		}

	}

}
