use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub key: InpStr <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { key, params } = [ params, key ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_rounds: u32 = ("NUM_ROUNDS=", 64, 1 ..= 64),
		pub num_rows: u32 = ("NUM_ROWS=", 128, 1 ..= 128),
	}
}
