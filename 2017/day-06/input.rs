use super::*;

use model::Banks;

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Input {
	pub banks: Banks,
	pub params: InputParams,
}

struct_parser_display! {
	Input { banks, params } = [ params, @delim "\t" banks ]
}

input_params! {
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct InputParams {
	}
}
