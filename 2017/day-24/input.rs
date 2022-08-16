use super::*;
use model::Component;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub comps: Vec <Component>,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}

impl Input {
	pub fn parse (input: & [& str]) -> GenResult <Self> {
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params, (@line_items comps));
			Ok (Self { comps, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for comp in self.comps.iter () {
			write! (formatter, "{}\n", comp) ?;
		}
		Ok (())
	}
}
