use super::*;

use model::Fold;
use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub dots: Vec <Pos>,
	pub folds: Vec <Fold>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { dots, folds, params } = [
		params,
		@lines dots { Pos { y, x } = [ x, ",", y ] }, "\n",
		"\n",
		@lines folds,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
