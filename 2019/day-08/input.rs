use super::*;
use model::Pixel;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub pixels: Vec <Pixel>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { pixels, params } = [ params, @collect pixels ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
