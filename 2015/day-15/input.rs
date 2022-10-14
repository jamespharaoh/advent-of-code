use super::*;

use model::Ingredient;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub ingrs: Vec <Ingredient <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { ingrs, params } = [ params, @lines ingrs ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
