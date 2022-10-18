use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub serial: i32,
	pub params: InputParams,
}

struct_parser_display! {
	Input { serial, params } = [ params, serial ]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
		pub grid_size: i16 = ("GRID_SIZE=", 300, 2 .. ),
	}
}
