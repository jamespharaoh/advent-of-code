use super::*;

use model::Grid;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub grid: Grid,
	pub params: InputParams,
}

struct_parser_display! {
	Input { grid, params } = [ params, grid = grid_parse ]
}

fn grid_parse (parser: & mut Parser) -> ParseResult <Grid> {
	Grid::parse_with_fn (parser, |parser| {
		if ! matches! (parser.peek (), Some ('0' ..= '9')) { return Err (parser.err ()) }
		Ok (parser.next ().unwrap ().to_digit (10).unwrap ().pan_u8 ())
	})
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
