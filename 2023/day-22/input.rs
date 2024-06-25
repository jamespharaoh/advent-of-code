use super::*;

use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub blocks: Vec <Block>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { blocks, params } = [ params, @lines blocks ]
}

#[ derive (Clone, Copy, Debug) ]
pub struct Block {
	pub start: Pos,
	pub end: Pos,
}

struct_parser_display! {
	Block {
		start: Pos { x: start_x, y: start_y, z: start_z },
		end: Pos { x: end_x, y: end_y, z: end_z },
	} = [
		start_x, ",", start_y, ",", start_z, "~",
		end_x, ",", end_y, ",", end_z,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
