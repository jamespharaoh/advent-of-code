use super::*;

use model::Wire;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub wires: Vec <Wire <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { wires, params } = [ params, @lines wires ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
