use super::*;

use model::Orbit;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub orbits: Vec <Orbit <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { orbits, params } = [ params, @lines orbits ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
