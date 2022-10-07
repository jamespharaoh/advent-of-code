use super::*;

use model::Reindeer;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub deers: Vec <Reindeer <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { deers, params } = [ params, @lines deers ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub race_time: u32 = ("RACE_TIME=", 2503, 1_u32 .. ),
	}
}
