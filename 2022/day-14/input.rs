use super::*;

use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub traces: Vec <Trace>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { traces, params } = [ params, @lines traces ]
}

#[ derive (Clone, Debug) ]
pub struct Trace {
	pub points: Vec <Pos>,
}

struct_parser_display! {
	Trace { points } = [ @delim " -> " points { Pos { y, x } = [ x, ",", y ] } ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
