use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub rucksacks: Vec <InpStr <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { rucksacks, params } = [
		params,
		@lines rucksacks {
			items = [ @str items = (|ch| { ch.is_ascii_alphabetic () }, 2 .. ) ],
		},
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
