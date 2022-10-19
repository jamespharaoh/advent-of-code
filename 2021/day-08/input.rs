use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub displays: Vec <InputDisplay <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { displays, params } = [ params, @lines displays ]
}

#[ derive (Clone, Debug) ]
pub struct InputDisplay <'inp> {
	pub samples: [InpStr <'inp>; 10],
	pub value: [InpStr <'inp>; 4],
}

struct_parser_display! {
	input_lifetime = 'inp;
	InputDisplay <'inp> { samples, value } = [
		@array_delim " " samples {
			digit = [ @str digit = ('a' ..= 'g', 1 ..= 7) ],
		},
		" | ",
		@array_delim " " value {
			digit = [ @str digit = ('a' ..= 'g', 1 ..= 7) ],
		},
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub use_solver: bool = ("USE_SOLVER=", false, false ..= true ),
	}
}
