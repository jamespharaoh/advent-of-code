use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub pairs: Vec <(InpStr <'inp>, InpStr <'inp>, DiffSign, i32)>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { pairs, params } = [
		params, @lines pairs {
			type = (InpStr, InpStr, DiffSign, i32);
			(person_0, person_1, sign, diff) = [
				@str person_0 = (|ch| { ch.is_ascii_alphabetic () }, 1 ..= 10),
				" would ", sign, " ",
				diff = 0_i32 ..= i32::MAX,
				" happiness units by sitting next to ",
				@str person_1 = (|ch| { ch.is_ascii_alphabetic () }, 1 ..= 10),
				".",
			],
		},
	]
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug) ]
	pub enum DiffSign {
		Gain = [ "gain" ],
		Lose = [ "lose" ],
	}
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
