use super::*;
use model::VHexDir;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub steps: Vec <VHexDir>,
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
			parse! (parser, params, @delim "," steps);
			Ok (Self { steps, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for item in Itertools::intersperse (
				self.steps.iter ().copied ().map (Either::Left),
				Either::Right (",")) {
			write! (formatter, "{}", item) ?;
		}
		Ok (())
	}
}
