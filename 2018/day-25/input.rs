use super::*;
use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub coords: Vec <Pos>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { coords, params } = [
		params,
		@lines coords {
			Pos { x, y, z, t } = [ x, ",", y, ",", z, ",", t ],
		},
	]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}
