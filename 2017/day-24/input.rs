use super::*;

use model::Port;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub comps: Vec <Component>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { comps, params } = [ params, @lines comps ]
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Component {
	pub port_0: Port,
	pub port_1: Port,
}

struct_parser_display! {
	Component { port_0, port_1 } = [ port_0, "/", port_1 ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
