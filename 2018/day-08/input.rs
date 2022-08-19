use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub data: Vec <u8>,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_workers: u32 = ("NUM_WORKERS=", 5, 1_u32 ..= 10),
		pub extra_time: u32 = ("EXTRA_TIME=", 60, 0_u32 ..= 60),
	}
}

impl Input {
	pub fn parse (input: & [& str]) -> GenResult <Self> {
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params, @delim " " data);
			Ok (Self { data, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for item in Itertools::intersperse (
				self.data.iter_vals ().map (Either::Left),
				Either::Right (" ")) {
			Display::fmt (& item, formatter) ?;
		}
		write! (formatter, "\n") ?;
		Ok (())
	}
}
