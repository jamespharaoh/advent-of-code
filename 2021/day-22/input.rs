use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub steps: Vec <InputStep>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { steps, params } = [ params, @lines steps ]
}

#[ derive (Clone, Copy, Debug) ]
pub struct InputStep {
	pub state: bool,
	pub cube: InputCube,
}

struct_parser_display! {
	InputStep { state, cube } = [
		state { true = [ "on" ], false = [ "off" ] }, " ",
		cube,
	]
}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
pub struct InputCube {
	pub x0: i32, pub x1: i32,
	pub y0: i32, pub y1: i32,
	pub z0: i32, pub z1: i32,
}

struct_parser_display! {
	InputCube { x0, x1, y0, y1, z0, z1 } = [
		"x=", x0, "..", x1, ",",
		"y=", y0, "..", y1, ",",
		"z=", z0, "..", z1,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
