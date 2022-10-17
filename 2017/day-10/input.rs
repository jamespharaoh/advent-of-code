use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub data: InpStr <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { data, params } = [ params, data ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub rounds_one: u32 = ("ROUNDS_ONE=", 1, 1 ..= 64),
		pub rounds_two: u32 = ("ROUNDS_TWO=", 64, 1 ..= 64),
	}
}
