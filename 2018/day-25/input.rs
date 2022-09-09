use super::*;
use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub coords: Vec <InputCoord>,
	pub params: InputParams,
}

struct_parser_display! {
	Input {
		coords,
		params,
	} = [
		params,
		@lines coords,
	]
}

#[ derive (Clone, Copy, Debug) ]
pub struct InputCoord {
	pub coord: Pos,
}

struct_parser_display! {
	InputCoord { coord: Pos { x, y, z, t } } = [ x, ",", y, ",", z, ",", t ]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}
