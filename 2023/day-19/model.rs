use super::*;

pub type Val = u16;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Op {
		LessThan = [ "<" ],
		GreaterThan = [ ">" ],
	}
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Field {
		X = [ "x" ],
		M = [ "m" ],
		A = [ "a" ],
		S = [ "s" ],
	}
}
