use super::*;

use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub moons: Vec <Pos>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { moons, params } = [
		params,
		@lines moons {
			Pos { x, y, z } = [ "<x=", x, ", y=", y, ", z=", z, ">" ],
		},
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_steps_one: u32 = ("NUM_STEPS_ONE=", 1000, 1 .. ),
		pub num_steps_two: u32 = ("NUM_STEPS_TWO=", 500_000, 1 .. ),
	}
}
