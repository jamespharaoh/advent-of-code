use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub cnxns: Vec <InputConnection <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { cnxns, params } = [ params, @lines cnxns ]
}

#[ derive (Clone, Debug) ]
pub struct InputConnection <'inp> {
	pub cave_0: InpStr <'inp>,
	pub cave_1: InpStr <'inp>,
}

struct_parser_display! {
	input_lifetime = 'inp;
	InputConnection <'inp> { cave_0, cave_1 } = [
		@str cave_0 = (|ch| { ch.is_ascii_alphabetic () }, 1 .. ),
		"-",
		@str cave_1 = (|ch| { ch.is_ascii_alphabetic () }, 1 .. ),
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
