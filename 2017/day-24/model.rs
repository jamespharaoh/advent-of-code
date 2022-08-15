use super::*;

pub type Components = u64;
pub type Port = u16;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Component {
	pub port_0: Port,
	pub port_1: Port,
}

impl Display for Component {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "{}/{}", self.port_0, self.port_1) ?;
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for Component {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, port_0, "/", port_1);
		Ok (Self { port_0, port_1 })
	}
}
