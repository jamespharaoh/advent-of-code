use super::*;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Pixel {
		Black = [ "0" ],
		White = [ "1" ],
		Transparent = [ "2" ],
	}
}
