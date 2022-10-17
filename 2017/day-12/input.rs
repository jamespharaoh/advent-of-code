use super::*;
use model::Village;

pub type InputPipes = Vec <InputPipe>;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub pipes: InputPipes,
	pub params: InputParams,
}

struct_parser_display! {
	Input { pipes, params } = [ params, @lines pipes ]
}

#[ derive (Clone, Debug) ]
pub struct InputPipe {
	pub left: Village,
	pub right: Vec <Village>,
}

struct_parser_display! {
	InputPipe { left, right } = [ left, " <-> ", @delim ", " right ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
