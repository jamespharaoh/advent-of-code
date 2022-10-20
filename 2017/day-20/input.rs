use super::*;

use model::Particle;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub particles: Vec <Particle>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { particles, params } = [ params, @lines particles ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
