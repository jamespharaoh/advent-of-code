use super::*;
use nums::IntConv;
use ops::{ BitAnd, BitOr, BitXor, Not };

#[ derive (Eq, PartialEq) ]
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
	message: ArrayVec <u8, 64>,
	len: usize,
}

type State = [WrappingU32; 4];

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
	pub fn from_hex (input: & str) -> GenResult <Self> {
		let input_len = input.chars ().count ();
		if input_len != 32 { Err (format! ("Expected 32 chars, not {}", input_len)) ? }
		let mut result = [0; 16];
		let mut input_iter = input.chars ();
		for result_ch in result.iter_mut () {
			let (high_ch, low_ch) = input_iter.next_tuple ().unwrap ();
			let decode = |ch: char| ch.to_digit (16).ok_or (format! ("Invalid hex: {}", ch));
			* result_ch = (decode (high_ch) ?.as_u8 ()) << 4_i32 | decode (low_ch) ?.as_u8 ();
		}
		Ok (Self (result))
	}

	#[ inline ]
	#[ must_use ]
	pub fn num_zeros (& self) -> u8 {
		let mut result = 0;
		for byte in self.0.iter_vals () {
			if byte & 0xf0 != 0 { break }
			result += 1;
			if byte & 0x0f != 0 { break }
			result += 1;
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
		write! (formatter, "md5::Output (\"") ?;
		for idx in 0 .. 16 {
			write! (formatter, "{:02x}", self.0 [idx]) ?;
		}
		write! (formatter, "\")") ?;
		Ok (())
	}

}

impl Display for Output {

	#[ allow (clippy::missing_inline_in_public_items) ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for idx in 0 .. 16 {
			write! (formatter, "{:02x}", self.0 [idx]) ?;
		}
		Ok (())
	}

}

impl MD5 {

	#[ inline ]
	#[ must_use ]
	pub fn new () -> Self {
		Self {
			state: INITIAL_STATE,
			message: ArrayVec::new (),
			len: 0,
		}
	}

	#[ allow (clippy::missing_inline_in_public_items) ]
	pub fn update (& mut self, mut message: & [u8]) {

		// iterate over message

		while ! message.is_empty () {

			// copy max sized chunk to buffer

			let bytes = cmp::min (self.message.remaining_capacity (), message.len ());
			self.message.extend (message.iter ().copied ().take (bytes));
			message = & message [bytes .. ];
			self.len = self.len.wrapping_add (bytes << 3);

			// stop now if buffer is part filled

			if ! self.message.is_full () { return }

			// consume buffer

			self.apply ();

		}

	}

	#[ allow (clippy::missing_inline_in_public_items) ]
	#[ must_use ]
	pub fn finish (mut self) -> Output {

		// remember the length before padding

		let mut len = self.len;

		// add one then zeros

		self.update (& [ 0x80 ]);
		while self.message.remaining_capacity () != 8 {
			self.update (& [ 0x00 ]);
		}

		// then the length

		for _ in 0_i32 .. 8_i32 {
			self.update (& [ (len & 0xff).as_u8 () ]);
			len >>= 8_i32;
		}

		// convert result words to byte array

		assert! (self.message.is_empty ());
		let mut result = [0; 16];
		for src_idx in 0 .. 4 {
			let dst_idx = src_idx << 2_i32;
			result [dst_idx] = (self.state [src_idx].0 & 0xff).as_u8 ();
			result [dst_idx + 1] = (self.state [src_idx].0 >> 8_i32 & 0xff).as_u8 ();
			result [dst_idx + 2] = (self.state [src_idx].0 >> 16_i32 & 0xff).as_u8 ();
			result [dst_idx + 3] = (self.state [src_idx].0 >> 24_i32 & 0xff).as_u8 ();
		}

		Output (result)

	}

	fn apply (& mut self) {

		// convert message buffer into words

		assert! (self.message.is_full ());
		let message = {
			let mut message = [WrappingU32 (0); 16];
			#[ allow (clippy::needless_range_loop) ]
			for dst_idx in 0 .. 16 {
				let src_idx = dst_idx << 2_i32;
				message [dst_idx] = WrappingU32 (
					(self.message [src_idx]).as_u32 ()
						| (self.message [src_idx + 1].as_u32 ()) << 8
						| (self.message [src_idx + 2].as_u32 ()) << 16
						| (self.message [src_idx + 3].as_u32 ()) << 24
				);
			}
			message
		};

		// apply rounds as specified

		let [mut a, mut b, mut c, mut d] = self.state;
		for op in 0 .. 16 {
			let func = ((b & c) | (! b & d)) + a + WrappingU32 (ADDS [op]) + message [op];
			(a, b, c, d) = (d, b + func.rotate_left (ROTATES [op].as_u32 ()), b, c);
		}
		for op in 16 .. 32 {
			let func = ((d & b) | (! d & c)) + a + WrappingU32 (ADDS [op]) + message [(5 * op + 1) % 16];
			(a, b, c, d) = (d, b + func.rotate_left (ROTATES [op].as_u32 ()), b, c);
		}
		for op in 32 .. 48 {
			let func = (b ^ c ^ d) + a + WrappingU32 (ADDS [op]) + message [(3 * op + 5) % 16];
			(a, b, c, d) = (d, b + func.rotate_left (ROTATES [op].as_u32 ()), b, c);
		}
		for op in 48 .. 64 {
			let func = (c ^ (b | ! d)) + a + WrappingU32 (ADDS [op]) + message [7 * op % 16];
			(a, b, c, d) = (d, b + func.rotate_left (ROTATES [op].as_u32 ()), b, c);
		}
		self.state = [
			self.state [0] + a,
			self.state [1] + b,
			self.state [2] + c,
			self.state [3] + d,
		];

		// clear buffer

		self.message.clear ();

	}

}

impl Default for MD5 {

	#[ inline ]
	fn default () -> Self {
		Self::new ()
	}

}

#[ derive (Clone, Copy) ]
struct WrappingU32 (u32);

impl WrappingU32 {
	const fn rotate_left (self, arg: u32) -> Self { Self (self.0.rotate_left (arg)) }
}

impl Add for WrappingU32 {
	type Output = Self;
	fn add (self, other: Self) -> Self { Self (self.0.wrapping_add (other.0)) }
}

impl BitAnd for WrappingU32 {
	type Output = Self;
	fn bitand (self, other: Self) -> Self { Self (self.0 & other.0) }
}

impl BitOr for WrappingU32 {
	type Output = Self;
	fn bitor (self, other: Self) -> Self { Self (self.0 | other.0) }
}

impl BitXor for WrappingU32 {
	type Output = Self;
	fn bitxor (self, other: Self) -> Self { Self (self.0 ^ other.0) }
}

impl Not for WrappingU32 {
	type Output = Self;
	fn not (self) -> Self { Self (! self.0) }
}

const INITIAL_STATE: State = [
	WrappingU32 (0x_6745_2301), WrappingU32 (0x_efcd_ab89),
	WrappingU32 (0x_98ba_dcfe), WrappingU32 (0x_1032_5476),
];

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
