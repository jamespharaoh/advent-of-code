//! Data structures to model the puzzle input

use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub stream: Vec <Token>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { stream, params } = [ params, @collect stream ]
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug) ]
	pub enum Token {
		Letter (ch: char) = [ ch = ('A' ..= 'Z') ],
		Repeat (len: u16, num: u8) = [ "(", len, "x", num, ")" ],
	}
}

impl Token {

	#[ must_use ]
	pub fn display_len (self) -> u32 {
		fn num_digits (num: u32) -> u32 {
			match num {
				0 ..= 9 => 1,
				10 ..= 99 => 2,
				100 ..= 999 => 3,
				1_000 ..= 9_999 => 4,
				10_000 ..= 99_999 => 5,
				100_000 ..= 999_999 => 6,
				1_000_000 ..= 9_999_999 => 7,
				10_000_000 ..= 99_999_999 => 8,
				100_000_000 ..= 999_999_999 => 9,
				1_000_000_000 ..= 0x_ffff_ffff => 10,
			}
		}
		match self {
			Self::Letter (_) => 1,
			Self::Repeat (len, num) =>
				3 + num_digits (len.pan_u32 ()) + num_digits (num.pan_u32 ()),
		}
	}

}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
