use super::*;

use model::Value;

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Input {
	pub rows: Vec <Row>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { rows, params } = [ params, @lines rows ]
}

wrapper_deref_mut! {
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Row {
		pub cells: Vec <Value>,
	}
}

struct_parser_display! {
	Row { cells } = [ @delim "\t" cells = (1 .. ) ]
}

input_params! {
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct InputParams {
	}
}
