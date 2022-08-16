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
		parse! (parser, depth, ": ", range);
		Ok (Self { depth, range })
	}
}
