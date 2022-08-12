use super::*;

pub type LayerDepth = u8;
pub type LayerRange = u8;

#[ derive (Clone, Debug, Eq, Ord, PartialEq, PartialOrd) ]
pub struct Layer {
	pub depth: LayerDepth,
	pub range: LayerRange,
}

impl Display for Layer {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "{}: {}", self.depth, self.range) ?;
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for Layer {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		let depth: LayerDepth = parser.uint () ?;
		let range: LayerRange = parser.expect (": ") ?.uint () ?;
		if range < 1 { return Err (parser.err ()) }
		Ok (Self { depth, range })
	}
}
