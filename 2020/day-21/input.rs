use super::*;

use model::Food;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub foods: Vec <Food <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { foods, params } = [ params, @lines foods ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
