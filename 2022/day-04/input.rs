use super::*;

use model::Val;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub pairs: Vec <InputPair>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { pairs, params } = [ params, @lines pairs ]
}

#[ derive (Clone, Copy, Debug) ]
pub struct InputPair {
	pub first_start: Val,
	pub first_end: Val,
	pub second_start: Val,
	pub second_end: Val,
}

struct_parser_display! {
	InputPair { first_start, first_end, second_start, second_end } = [
		first_start, "-", first_end, ",", second_start, "-", second_end,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
