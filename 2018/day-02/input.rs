use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub box_ids: Vec <InpStr <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { box_ids, params } = [
		params,
		@lines box_ids { box_id = [
			@str box_id = (|ch| { ch.is_ascii_lowercase () }, 1 ..= 32),
		] },
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
