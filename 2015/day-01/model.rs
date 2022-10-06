use super::*;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Dir {
		Up = [ "(" ],
		Down = [ ")" ],
	}
}

impl Dir {

	#[ must_use ]
	pub const fn val (& self) -> i32 {
		match * self { Self::Up => 1_i32, Self::Down => -1_i32 }
	}

}
