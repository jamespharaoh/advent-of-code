use super::*;

/// Trait implemented by types which can be produced by [`Parser::item`]
///
pub trait FromParser <'inp>: Sized {

	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self>;

	#[ inline ]
	fn parse_from_str (input: & 'inp str) -> GenResult <Self> {
		Parser::wrap_auto (input, Parser::item)
	}

	#[ inline ]
	fn parse_from_lines (input: & 'inp [& 'inp str]) -> GenResult <Self> {
		Parser::wrap_lines (input, Parser::item)
	}

}

macro_rules! from_parser_impl {
	( $name:ident, $method:ident ) => {
		impl <'inp> FromParser <'inp> for $name {
			#[ inline ]
			fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <$name> {
				parser.$method ()
			}
		}
	};
}

from_parser_impl! (i8, int);
from_parser_impl! (i16, int);
from_parser_impl! (i32, int);
from_parser_impl! (i64, int);
from_parser_impl! (i128, int);
from_parser_impl! (isize, int);

from_parser_impl! (u8, uint);
from_parser_impl! (u16, uint);
from_parser_impl! (u32, uint);
from_parser_impl! (u64, uint);
from_parser_impl! (u128, uint);
from_parser_impl! (usize, uint);

from_parser_impl! (char, expect_next);

impl <'inp> FromParser <'inp> for bool {

	#[ inline ]
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parser.any ()
			.of (|parser| { parser.expect ("true") ?; Ok (true) })
			.of (|parser| { parser.expect ("false") ?; Ok (false) })
			.done ()
	}

}

impl <'inp> FromParser <'inp> for InpStr <'inp> {

	#[ inline ]
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		Ok (parser.take_rest ())
	}

}

impl <'inp> FromParser <'inp> for Rc <str> {

	#[ inline ]
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		let inp_str = parser.take_rest ();
		Ok (Self::from (& * inp_str))
	}

}
