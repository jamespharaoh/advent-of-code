use super::*;
use model::Point;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub points: Vec <Point>,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}

impl <'inp> FromParser <'inp> for Input {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, params, (@line_items points));
		Ok (Self { points, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for point in self.points.iter () {
			write! (formatter, "{}\n", point) ?;
		}
		Ok (())
	}
}
