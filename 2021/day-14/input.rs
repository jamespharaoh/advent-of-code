use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub template: InpStr <'inp>,
	pub rules: Vec <((char, char), char)>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { template, rules, params } = [
		params,
		@str template = (|ch| { ch.is_ascii_uppercase () }, 1 .. ), "\n",
		"\n",
		@lines rules { ((left, right), insert) = [ left, right, " -> ", insert ] },
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
