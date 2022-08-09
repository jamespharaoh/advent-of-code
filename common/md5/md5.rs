use std::cmp;
use std::fmt::{ self, Debug, Display, Write as _ };
use std::ops::Index;

#[ cfg (test) ] use aoc_misc::*;
use aoc_nums as nums;
use nums::IntConv;

#[ derive (Clone, Copy, Default, Eq, PartialEq) ]
pub struct Output ([u8; 16]);

#[ inline ]
#[ must_use ]
pub fn md5_hash (input: & [u8]) -> Output {
	let mut md5 = MD5::new ();
	md5.update (input);
	md5.finish ()
}

pub struct MD5 {
	state: State,
	message: [u8; 64],
	message_len: usize,
	len: usize,
}

type State = [u32; 4];

impl Output {

	#[ inline ]
	#[ must_use ]
	pub const fn len (& self) -> usize {
		self.0.len ()
	}

	#[ inline ]
	#[ must_use ]
	pub const fn is_empty (& self) -> bool {
		self.0.is_empty ()
	}

	#[ allow (clippy::missing_inline_in_public_items) ]
	pub fn from_hex (input: & str) -> Result <Self, String> {
		let input_len = input.chars ().count ();
		if input_len != 32 { return Err (format! ("Expected 32 chars, not {}", input_len)) }
		let mut result = [0; 16];
		let mut input_iter = input.chars ();
		for result_ch in result.iter_mut () {
			let high_ch = input_iter.next ().unwrap ();
			let low_ch = input_iter.next ().unwrap ();
			let decode = |ch: char| ch.to_digit (16).ok_or (format! ("Invalid hex: {}", ch));
			* result_ch = (decode (high_ch) ?.as_u8 ()) << 4_i32 | decode (low_ch) ?.as_u8 ();
		}
		Ok (Self (result))
	}

	#[ inline ]
	#[ must_use ]
	pub fn num_zeros (& self) -> u8 {
		let mut result = 0;
		for byte in self.0.iter ().copied () {
			if byte & 0xf0 != 0 { break }
			result += 1;
			if byte & 0x0f != 0 { break }
			result += 1;
		}
		result
	}

	#[ inline ]
	#[ must_use ]
	pub fn as_hex_bytes (& self) -> [u8; 32] {
		#[ inline ]
		const fn nibble_as_hex (nibble: u8) -> u8 {
			if nibble >= 10 { b'a' + nibble - 10 } else { b'0' + nibble }
		}
		let mut result = [0; 32];
		let mut result_iter = result.iter_mut ();
		for byte in self.0.iter () {
			* result_iter.next ().unwrap () = nibble_as_hex (byte >> 4_u32);
			* result_iter.next ().unwrap () = nibble_as_hex (byte & 0xf);
		}
		result
	}

}

impl Index <usize> for Output {

	type Output = u8;

	#[ inline ]
	fn index (& self, idx: usize) -> & u8 {
		& self.0 [idx]
	}

}

impl Debug for Output {

	#[ allow (clippy::missing_inline_in_public_items) ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		formatter.write_str ("md5::Output (\"") ?;
		Display::fmt (self, formatter) ?;
		formatter.write_str ("\")") ?;
		Ok (())
	}

}

impl Display for Output {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for byte in self.0.iter ().copied () {
			let byte = byte.as_u32 ();
			formatter.write_char (char::from_digit (byte >> 4, 16).unwrap ()) ?;
			formatter.write_char (char::from_digit (byte & 0xf, 16).unwrap ()) ?;
		}
		Ok (())
	}

}

impl MD5 {

	#[ inline ]
	#[ must_use ]
	pub const fn new () -> Self {
		Self {
			state: INITIAL_STATE,
			message: [0; 64],
			message_len: 0,
			len: 0,
		}
	}

	#[ inline ]
	pub fn reset (& mut self) {
		self.state = INITIAL_STATE;
		self.message_len = 0;
		self.len = 0;
	}

	#[ inline ]
	pub fn push (& mut self, byte: u8) {
		self.message [self.message_len] = byte;
		self.message_len += 1;
		if self.message_len == 64 {
			self.apply ();
		}
	}

	#[ inline ]
	pub fn update (& mut self, mut message: & [u8]) {

		// iterate over message

		while ! message.is_empty () {

			// copy max sized chunk to buffer

			let bytes = cmp::min (64 - self.message_len, message.len ());
			self.message [self.message_len .. self.message_len + bytes].copy_from_slice (& message [ .. bytes]);
			self.message_len += bytes;
			message = & message [bytes .. ];
			self.len = self.len.wrapping_add (bytes << 3);

			// stop now if buffer is part filled

			if self.message_len != 64 { return }

			// consume buffer

			self.apply ();

		}

	}

	#[ inline ]
	#[ must_use ]
	pub fn finish (mut self) -> Output {
		self.finish_real ()
	}

	#[ inline ]
	pub fn finish_reset (& mut self) -> Output {
		let output = self.finish_real ();
		self.reset ();
		output
	}

	#[ allow (clippy::missing_inline_in_public_items) ]
	#[ must_use ]
	fn finish_real (& mut self) -> Output {

		// remember the length before padding

		let len = self.len;

		// add one then zeros, leaving exactly 56 bytes buffered

		self.push (0x80);
		if self.message_len > 56 {
			for idx in self.message_len .. 64 { self.message [idx] = 0; }
			self.message_len = 64;
			self.apply ();
		}
		for idx in self.message_len .. 56 { self.message [idx] = 0; }
		self.message_len = 56;

		// then the length, which takes us to 64 bytes

		self.message [56 .. 64].copy_from_slice (& len.to_le_bytes ());
		self.message_len += 8;
		self.apply ();

		// convert result words to byte array

		assert! (self.message_len == 0);

		let mut result = [0; 16];
		for src_idx in 0 .. 4 {
			let dst_idx = src_idx << 2_i32;
			result [dst_idx] = (self.state [src_idx] & 0xff).as_u8 ();
			result [dst_idx + 1] = (self.state [src_idx] >> 8_i32 & 0xff).as_u8 ();
			result [dst_idx + 2] = (self.state [src_idx] >> 16_i32 & 0xff).as_u8 ();
			result [dst_idx + 3] = (self.state [src_idx] >> 24_i32 & 0xff).as_u8 ();
		}

		Output (result)

	}

	fn apply (& mut self) {

		// convert message buffer into words

		assert! (self.message_len == 64);

		let mut message = [0; 16];
		#[ allow (clippy::needless_range_loop) ]
		for dst_idx in 0_usize .. 16_usize {
			let src_idx = dst_idx << 2_u32;
			message [dst_idx] = u32::from_le_bytes (
				self.message [src_idx .. src_idx + 4].try_into ().unwrap ());
		}

		// apply rounds as specified

		let [mut a, mut b, mut c, mut d] = self.state;

		for op in 0 .. 16 {
			let func = ((b & c) | (! b & d))
				.wrapping_add (a)
				.wrapping_add (ADDS [op])
				.wrapping_add (message [op]);
			(a, b, c, d) = (d, b.wrapping_add (func.rotate_left (ROTATES [op].as_u32 ())), b, c);
		}

		for op in 16 .. 32 {
			let func = ((d & b) | (! d & c))
				.wrapping_add (a)
				.wrapping_add (ADDS [op])
				.wrapping_add (message [(5 * op + 1) % 16]);
			(a, b, c, d) = (d, b.wrapping_add (func.rotate_left (ROTATES [op].as_u32 ())), b, c);
		}

		for op in 32 .. 48 {
			let func = (b ^ c ^ d)
				.wrapping_add (a)
				.wrapping_add (ADDS [op])
				.wrapping_add (message [(3 * op + 5) % 16]);
			(a, b, c, d) = (d, b.wrapping_add (func.rotate_left (ROTATES [op].as_u32 ())), b, c);
		}

		for op in 48 .. 64 {
			let func = (c ^ (b | ! d))
				.wrapping_add (a)
				.wrapping_add (ADDS [op])
				.wrapping_add (message [7 * op % 16]);
			(a, b, c, d) = (d, b.wrapping_add (func.rotate_left (ROTATES [op].as_u32 ())), b, c);
		}

		self.state = [
			self.state [0].wrapping_add (a),
			self.state [1].wrapping_add (b),
			self.state [2].wrapping_add (c),
			self.state [3].wrapping_add (d),
		];

		// clear buffer

		self.message_len = 0;

	}

}

impl Default for MD5 {

	#[ inline ]
	fn default () -> Self {
		Self::new ()
	}

}

const INITIAL_STATE: State = [ 0x_6745_2301, 0x_efcd_ab89, 0x_98ba_dcfe, 0x_1032_5476 ];

const ROTATES: [u8; 64] = [
	7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
	5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
	4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
	6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const ADDS: [u32; 64] = [
	0x_d76a_a478, 0x_e8c7_b756, 0x_2420_70db, 0x_c1bd_ceee, 0x_f57c_0faf, 0x_4787_c62a,
	0x_a830_4613, 0x_fd46_9501, 0x_6980_98d8, 0x_8b44_f7af, 0x_ffff_5bb1, 0x_895c_d7be,
	0x_6b90_1122, 0x_fd98_7193, 0x_a679_438e, 0x_49b4_0821, 0x_f61e_2562, 0x_c040_b340,
	0x_265e_5a51, 0x_e9b6_c7aa, 0x_d62f_105d, 0x_0244_1453, 0x_d8a1_e681, 0x_e7d3_fbc8,
	0x_21e1_cde6, 0x_c337_07d6, 0x_f4d5_0d87, 0x_455a_14ed, 0x_a9e3_e905, 0x_fcef_a3f8,
	0x_676f_02d9, 0x_8d2a_4c8a, 0x_fffa_3942, 0x_8771_f681, 0x_6d9d_6122, 0x_fde5_380c,
	0x_a4be_ea44, 0x_4bde_cfa9, 0x_f6bb_4b60, 0x_bebf_bc70, 0x_289b_7ec6, 0x_eaa1_27fa,
	0x_d4ef_3085, 0x_0488_1d05, 0x_d9d4_d039, 0x_e6db_99e5, 0x_1fa2_7cf8, 0x_c4ac_5665,
	0x_f429_2244, 0x_432a_ff97, 0x_ab94_23a7, 0x_fc93_a039, 0x_655b_59c3, 0x_8f0c_cc92,
	0x_ffef_f47d, 0x_8584_5dd1, 0x_6fa8_7e4f, 0x_fe2c_e6e0, 0x_a301_4314, 0x_4e08_11a1,
	0x_f753_7e82, 0x_bd3a_f235, 0x_2ad7_d2bb, 0x_eb86_d391,
];

#[ cfg (test) ]
mod tests {

	use super::*;

	const LOREM_IPSUM: & str =
		"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut at tempus ligula, ac \
		pellentesque leo. Nullam molestie justo sit amet neque venenatis, at laoreet urna \
		mollis. Mauris eget mollis quam. Maecenas ultricies odio dolor, id luctus lorem \
		aliquam at. Vestibulum est lectus, egestas vehicula mi vel, pharetra elementum \
		ligula. Nam cursus, magna vitae sodales pretium, metus ligula facilisis nisl, pretium \
		accumsan leo justo sed elit. Vestibulum efficitur justo quis molestie luctus. Aliquam \
		volutpat at quam quis egestas. Proin et turpis nec lacus maximus iaculis. Donec vitae \
		massa magna. Nulla pulvinar eleifend erat et fringilla.";

	macro_rules! assert_md5 {
		( $expect:expr , $input:expr ) => {
			assert_eq! (
				Output::from_hex ($expect).unwrap (),
				md5_hash ($input.as_bytes ()));
		}
	}

	#[ test ]
	fn test_md5_hash () {
		assert_md5! ("d41d8cd98f00b204e9800998ecf8427e", "");
		assert_md5! ("6cd3556deb0da54bca060b4c39479839", "Hello, world!");
		assert_md5! ("7bb31841cf426a6de079421b2590cf82", "The Ideal Stocking Stuffer");
		assert_md5! ("777f5a5ebdeab74ab0299512f9688be0", LOREM_IPSUM);
	}

	#[ test ]
	fn test_output () {
		let output = Output::from_hex ("0123456789abcdef0123456789abcdef").unwrap ();
		assert_eq! (16, output.len ());
		assert_eq! (false, output.is_empty ());
		assert_eq! (0x01, output [0]);
		assert_eq! (0xef, output [15]);
		assert_eq! ("md5::Output (\"0123456789abcdef0123456789abcdef\")", format! ("{:?}", output));
		assert_eq! ("0123456789abcdef0123456789abcdef", format! ("{}", output));
		assert_err! ("Expected 32 chars, not 31", Output::from_hex ("0123456789abcdef0123456789abcde"));
		assert_err! ("Expected 32 chars, not 33", Output::from_hex ("0123456789abcdef0123456789abcdef0"));
		assert_err! ("Invalid hex: X", Output::from_hex ("0123456789abcdeX0123456789abcdef"));
	}

}
