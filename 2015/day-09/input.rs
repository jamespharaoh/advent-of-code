use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub dists: Vec <(InpStr <'inp>, InpStr <'inp>, u32)>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { dists, params } = [
		params, @lines dists { (place_0, place_1, dist) = [
			@str place_0 = (|ch| { ch.is_ascii_alphabetic () }, 1 ..= 20), " to ",
			@str place_1 = (|ch| { ch.is_ascii_alphabetic () }, 1 ..= 20), " = ",
			dist,
		] },
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
