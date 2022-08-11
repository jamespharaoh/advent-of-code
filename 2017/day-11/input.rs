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
	pub fn parse (mut input: & [& str]) -> GenResult <Self> {
		let params = InputParams::parse (& mut input) ?;
		if input.len () != 1 {
			return Err ("Input must be exactly one line".into ());
		}
		let steps = Parser::wrap_auto (input [0], |parser| {
			parser.delim_items (",")
		}) ?;
		Ok (Self { steps, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for item in Itertools::intersperse (
				self.steps.iter_vals ().map (Either::Left),
				Either::Right (",")) {
			write! (formatter, "{}", item) ?;
		}
		Ok (())
	}
}
