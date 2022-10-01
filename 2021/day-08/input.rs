use super::*;

use model::Display;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub displays: Vec <Display>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { displays, params } = [ params, @lines displays ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub use_solver: bool = ("USE_SOLVER=", false, false ..= true ),
	}
}
