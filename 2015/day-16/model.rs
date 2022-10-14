//! Representation of the puzzle input, etc.
//!

use super::*;

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct AuntSue {
	pub number: u16,
	pub attrs: Vec <(Attr, u8)>,
}

struct_parser_display! {
	AuntSue { number, attrs } = [
		"Sue ", number, ": ",
		@delim ", " attrs {
			(attr, num) = [ attr, ": ", num ],
		},
	]
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Attr {
		Children = [ "children" ],
		Cats = [ "cats" ],
		Samoyeds = [ "samoyeds" ],
		Pomeranians = [ "pomeranians" ],
		Akitas = [ "akitas" ],
		Vizslas = [ "vizslas" ],
		Goldfish = [ "goldfish" ],
		Trees = [ "trees" ],
		Cars = [ "cars" ],
		Perfumes = [ "perfumes" ],
	}
}
