use super::*;

use model::Item;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub pairs: Vec <Pair>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { pairs, params } = [ params, @delim "\n\n" pairs ]
}

#[ derive (Clone, Debug) ]
pub struct Pair {
	pub one: Item,
	pub two: Item,
}

struct_parser_display! {
	Pair { one, two } = [ one, "\n", two ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
