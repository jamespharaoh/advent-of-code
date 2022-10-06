use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub strings: Vec <InpStr <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { strings, params } = [
		params,
		@lines strings { string = [
			@str string = (|ch| { ch.is_ascii_lowercase () }, 1 ..= 100),
		] },
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
