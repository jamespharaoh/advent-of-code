use super::*;
use model::Entry;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub entries: Vec <Entry>,
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
			parse! (parser, params, @lines entries);
			Ok (Self { entries, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for entry in self.entries.iter () {
			write! (formatter, "{}\n", entry) ?;
		}
		Ok (())
	}
}
