//! Data structures to model the puzzle input

use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub ops: Vec <ScrambleOp>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { ops, params } = [ params, @lines ops ]
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
	pub enum ScrambleOp {
		SwapPosns (pos_0: u32, pos_1: u32) = [
			"swap position ", pos_0, " with position ", pos_1,
		],
		SwapChars (ch_0: char, ch_1: char) = [
			"swap letter ", ch_0, " with letter ", ch_1,
		],
		RotLeft (num: u32) = [
			"rotate left ", num {
				num if (* num != 1) = [ num, " steps" ],
				num if (* num == 1) = [ num, " step" ],
			},
		],
		RotRight (num: u32) = [
			"rotate right ", num {
				num if (* num != 1) = [ num, " steps" ],
				num if (* num == 1) = [ num, " step" ],
			},
		],
		RotChar (ch: char) = [
			"rotate based on position of letter ", ch,
		],
		Reverse (pos_0: u32, pos_1: u32) = [
			"reverse positions ", pos_0, " through ", pos_1,
		],
		Move (pos_0: u32, pos_1: u32) = [
			"move position ", pos_0, " to position ", pos_1,
		],
	}
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub start_one: String = ("START_ONE=", "abcdefgh".to_owned (), .. ),
		pub start_two: String = ("START_TWO=", "fbgdceah".to_owned (), .. ),
	}
}
