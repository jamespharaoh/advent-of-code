use super::*;

use model::Val;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub elves: Vec <InputElf>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { elves, params } = [ params, @delim "\n\n" elves ]
}

#[ derive (Clone, Debug) ]
pub struct InputElf {
	pub items: Vec <Val>,
}

struct_parser_display! {
	InputElf { items } = [ @lines items ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
