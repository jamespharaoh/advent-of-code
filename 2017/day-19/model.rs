use super::*;

pub type Coord = u8;
pub type Dir = pos::Dir2d;
pub type Grid = GridBuf <Vec <Tile>, Pos, 2>;
pub type Pos = pos::PosRowCol <Coord>;
pub type Turn = pos::Turn2d;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, PartialEq) ]
	pub enum Tile {
		#[ default ]
		Empty = [ " " ],
		Horiz = [ "-" ],
		Vert = [ "|" ],
		Corner = [ "+" ],
		Letter (asc: u8) = [ asc = (letter_parse, letter_display) ],
	}
}

fn letter_parse (parser: & mut Parser) -> ParseResult <u8> {
	if ! matches! (parser.peek (), Some ('A' ..= 'Z')) { return Err (parser.err ()) }
	Ok (parser.next ().unwrap ().pan_u8 ())
}

fn letter_display (letter: & u8, formatter: & mut fmt::Formatter) -> fmt::Result {
	formatter.write_char (letter.pan_char ())
}
