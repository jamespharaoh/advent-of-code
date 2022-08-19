use super::*;
use model::Step;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub steps: Vec <Step>,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub reps_two: u64 = ("REPS_TWO=", 1_000_000_000, 2_u64 .. ),
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
				self.steps.iter ().map (Either::Left),
				Either::Right (",")) {
			Display::fmt (& item, formatter) ?;
		}
		Ok (())
	}
}
