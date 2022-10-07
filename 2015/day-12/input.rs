use super::*;

use model::Json;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub json: Json <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { json, params } = [ params, json ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
