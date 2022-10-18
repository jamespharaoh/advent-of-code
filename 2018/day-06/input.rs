use super::*;

use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub posns: Vec <Pos>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { posns, params } = [
		params,
		@lines posns { Pos { y, x } = [ x = 0 ..= 399, ", ", y = 0 ..= 399 ] },
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub dist_two: u32 = ("DIST_TWO=", 10_000, 1 ..= 100_000),
	}
}
