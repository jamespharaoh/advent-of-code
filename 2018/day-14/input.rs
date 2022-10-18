use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub value: InpStr <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { value, params } = [
		params,
		@str value = (|ch| { ch.is_ascii_digit () }, 1 ..= 6),
	]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
		pub max_recipes: u32 = ("MAX_RECIPES=", 50_000_000, 1_u32 .. ),
	}
}
