use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub box_ids: Vec <InpStr <'inp>>,
	pub params: InputParams,
}

struct_display! {
	input_lifetime = 'inp;
	Input <'inp> { box_ids, params } = [ params, @lines box_ids ]
}

struct_parser! {
	input_lifetime = 'inp;
	Input <'inp> { box_ids, params } = [ params, @lines box_ids = parse_box_id ]
}

fn parse_box_id <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
	parser.take_rest_while (|ch| ch.is_ascii_lowercase (), .. )
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
