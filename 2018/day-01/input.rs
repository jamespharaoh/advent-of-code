use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub deltas: Vec <i32>,
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
			parse! (parser, params, (@line_items deltas));
			Ok (Self { deltas, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for delta in self.deltas.iter () {
			write! (formatter, "{}\n", delta) ?;
		}
		Ok (())
	}
}
