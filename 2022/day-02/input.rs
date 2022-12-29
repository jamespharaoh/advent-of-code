use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub strategy: Vec <(InputChoice, OutputChoice)>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { strategy, params } = [
		params,
		@lines strategy {
			type = (InputChoice, OutputChoice);
			(input, output) = [ input, " ", output ],
		},
	]
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug) ]
	pub enum InputChoice {
		A = [ "A" ],
		B = [ "B" ],
		C = [ "C" ],
	}
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug) ]
	pub enum OutputChoice {
		X = [ "X" ],
		Y = [ "Y" ],
		Z = [ "Z" ],
	}
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
