use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub replacements: Vec <(InpStr <'inp>, InpStr <'inp>)>,
	pub medicine: InpStr <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { replacements, medicine, params } = [
		params,
		@lines replacements {
			(from, to) = [
				@str from = (|ch| { ch.is_ascii_alphabetic () }, 1 .. ),
				" => ",
				@str to = (|ch| { ch.is_ascii_alphabetic () }, 1 .. ),
			],
		}, "\n",
		"\n",
		medicine,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
