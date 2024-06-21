use super::*;

use model::Dir;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub steps: Vec <Step>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { steps, params } = [ params, @lines steps ]
}

#[ derive (Clone, Debug) ]
pub struct Step {
	pub dir: Dir,
	pub num: u32,
	pub colour: u32,
}

struct_parser_display! {
	Step { dir, num, colour } = [
		dir {
			type = Dir;
			Dir::Up = [ "U" ],
			Dir::Down = [ "D" ],
			Dir::Left = [ "L" ],
			Dir::Right = [ "R" ],
		}, " ",
		num = (1 ..= 20), " (",
		colour = parse_colour, ")" ]
}

fn parse_colour (parser: & mut Parser) -> ParseResult <u32> {
	parser.expect ("#") ?;
	let val_str = parser.take_rest_while (|ch| ch.is_ascii_hexdigit (), 6 ..= 6) ?;
	Ok (u32::from_str_radix (& val_str, 16).map_err (GenError::from) ?)
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
