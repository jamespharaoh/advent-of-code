use super::*;

pub type Coord = u16;
pub type Pos = pos::PosRowCol <u16>;
pub type SeenGrid = GridBuf <Vec <bool>, Pos, 2>;
pub type TilesGrid = GridBuf <Vec <Tile>, Pos, 2>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, PartialEq) ]
	pub enum Tile {
		#[ default ]
		Open = [ "." ],
		Wall = [ "#" ],
		Num (id: u8) = [ id = tile_id_parse ],
	}
}

fn tile_id_parse (parser: & mut Parser) -> ParseResult <u8> {
	if ! matches! (parser.peek (), Some ('0' ..= '9')) { return Err (parser.err ()) }
	Ok (parser.next ().unwrap ().to_digit (10).unwrap ().pan_u8 ())
}
