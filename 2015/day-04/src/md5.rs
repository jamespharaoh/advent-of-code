use super::*;
use ops::{ BitAnd, BitOr, BitXor, Not };

#[ derive (Eq, PartialEq) ]
pub struct Output ([u8; 16]);

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
	pub fn len (& self) -> usize { self.0.len () }
	pub fn from_hex (input: & str) -> GenResult <Self> {
		let input_len = input.chars ().count ();
		if input_len != 32 { Err (format! ("Expected 32 chars, not {}", input_len)) ? }
		let mut result = [0; 16];
		let mut input_iter = input.chars ();
		for idx in 0 .. 16 {
			let (high_ch, low_ch) = input_iter.next_tuple ().unwrap ();
			let decode = |ch: char| ch.to_digit (16).ok_or (format! ("Invalid hex: {}", ch));
			result [idx] = (decode (high_ch) ? as u8) << 4 | decode (low_ch) ? as u8;
		}
		Ok (Output (result))
	}
}

impl Index <usize> for Output {
	type Output = u8;
	fn index (& self, idx: usize) -> & u8 {
		& self.0 [idx]
	}
}

impl Debug for Output {
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
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for idx in 0 .. 16 {
			write! (formatter, "{:02x}", self.0 [idx]) ?;
		}
		Ok (())
	}
}

impl MD5 {

	pub fn new () -> MD5 {
		MD5 {
			state: INITIAL_STATE,
			message: ArrayVec::new (),
			len: 0,
		}
	}

	pub fn update (& mut self, mut message: & [u8]) {
		while ! message.is_empty () {
			let bytes = cmp::min (self.message.remaining_capacity (), message.len ());
			self.message.extend (message.iter ().copied ().take (bytes));
			message = & message [bytes .. ];
			self.len = self.len.wrapping_add (bytes << 3);
			if ! self.message.is_full () { return }
			self.apply ();
		}
	}

	pub fn finish (mut self) -> Output {
		self.message.push (0x80);
		while self.message.remaining_capacity () != 8 {
			self.message.push (0x00);
			if self.message.is_full () { self.apply (); }
		}
		let mut len = self.len;
		for _ in 0 .. 8 {
			self.update (& [ (len & 0xff) as u8 ]);
			len >>= 8;
		}
		let mut result = [0; 16];
		for src_idx in 0 .. 4 {
			let dst_idx = src_idx << 2;
			result [dst_idx] = self.state [src_idx].0 as u8;
			result [dst_idx + 1] = (self.state [src_idx].0 >> 8) as u8;
			result [dst_idx + 2] = (self.state [src_idx].0 >> 16) as u8;
			result [dst_idx + 3] = (self.state [src_idx].0 >> 24) as u8;
		}
		Output (result)
	}

	fn apply (& mut self) {
		assert! (self.message.is_full ());
		let message = {
			let mut message = [WrappingU32 (0); 16];
			for dst_idx in 0 .. 16 {
				let src_idx = dst_idx << 2;
				message [dst_idx] = WrappingU32 (
					(self.message [src_idx]) as u32
						| (self.message [src_idx + 1] as u32) << 8
						| (self.message [src_idx + 2] as u32) << 16
						| (self.message [src_idx + 3] as u32) << 24
				);
			}
			message
		};
		let [mut a, mut b, mut c, mut d] = self.state;
		for op in 0 .. 16 {
			let func = ((b & c) | (! b & d)) + a + WrappingU32 (ADDS [op]) + message [op];
			(a, b, c, d) = (d, b + func.rotate_left (ROTATES [op] as u32), b, c);
		}
		for op in 16 .. 32 {
			let func = ((d & b) | (! d & c)) + a + WrappingU32 (ADDS [op]) + message [(5 * op + 1) % 16];
			(a, b, c, d) = (d, b + func.rotate_left (ROTATES [op] as u32), b, c);
		}
		for op in 32 .. 48 {
			let func = (b ^ c ^ d) + a + WrappingU32 (ADDS [op]) + message [(3 * op + 5) % 16];
			(a, b, c, d) = (d, b + func.rotate_left (ROTATES [op] as u32), b, c);
		}
		for op in 48 .. 64 {
			let func = (c ^ (b | ! d)) + a + WrappingU32 (ADDS [op]) + message [7 * op % 16];
			(a, b, c, d) = (d, b + func.rotate_left (ROTATES [op] as u32), b, c);
		}
		self.state = [
			self.state [0] + a,
			self.state [1] + b,
			self.state [2] + c,
			self.state [3] + d,
		];
		self.message.clear ();
	}

}

#[ derive (Clone, Copy) ]
struct WrappingU32 (u32);

impl WrappingU32 {
	fn rotate_left (self, arg: u32) -> Self { WrappingU32 (self.0.rotate_left (arg)) }
}

impl Add for WrappingU32 {
	type Output = Self;
	fn add (self, other: Self) -> Self { WrappingU32 (self.0.wrapping_add (other.0)) }
}

impl BitAnd for WrappingU32 {
	type Output = WrappingU32;
	fn bitand (self, other: Self) -> Self { WrappingU32 (self.0 & other.0) }
}

impl BitOr for WrappingU32 {
	type Output = WrappingU32;
	fn bitor (self, other: Self) -> Self { WrappingU32 (self.0 | other.0) }
}

impl BitXor for WrappingU32 {
	type Output = WrappingU32;
	fn bitxor (self, other: Self) -> Self { WrappingU32 (self.0 ^ other.0) }
}

impl Not for WrappingU32 {
	type Output = WrappingU32;
	fn not (self) -> Self { WrappingU32 (! self.0) }
}

const INITIAL_STATE: State = [
	WrappingU32 (0x67452301), WrappingU32 (0xefcdab89),
	WrappingU32 (0x98badcfe), WrappingU32 (0x10325476),
];

const ROTATES: [u8; 64] = [
	7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
	5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
	4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
	6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const ADDS: [u32; 64] = [
	0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
	0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
	0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
	0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
	0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
	0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
	0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
	0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
	0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
	0xeb86d391,
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

}
