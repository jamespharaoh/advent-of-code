use super::*;

use model::Board;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub call_order: Vec <u8>,
	pub boards: Vec <Board>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { call_order, boards, params } = [
		params,
		@delim "," call_order, "\n",
		"\n",
		@delim "\n\n" boards = (board_parse, board_display),
	 ]
}

fn board_parse (parser: & mut Parser) -> ParseResult <Board> {
	parser.nest (|parser| {
		parser.set_ignore_whitespace (true);
		parser.item ()
	})
}

fn board_display (board: & Board, formatter: & mut fmt::Formatter) -> fmt::Result {
	board.display_with_delim (" ", formatter)
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
