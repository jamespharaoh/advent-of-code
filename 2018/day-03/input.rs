use super::*;
use model::Claim;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub claims: Vec <Claim>,
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
			parse! (parser, params, (@line_items claims));
			Ok (Self { claims, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for claim in self.claims.iter () {
			write! (formatter, "{}\n", claim) ?;
		}
		Ok (())
	}
}
