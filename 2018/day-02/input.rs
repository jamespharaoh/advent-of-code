use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub box_ids: Vec <InpStr <'inp>>,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}

impl <'inp> Input <'inp> {
	pub fn parse (input: & 'inp [& 'inp str]) -> GenResult <Self> {
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params, (@lines box_ids = |ch| ch.is_ascii_lowercase ()));
			Ok (Self { box_ids, params })
		})
	}
}

impl <'inp> Display for Input <'inp> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for box_id in self.box_ids.iter () {
			write! (formatter, "{}\n", box_id) ?;
		}
		Ok (())
	}
}
