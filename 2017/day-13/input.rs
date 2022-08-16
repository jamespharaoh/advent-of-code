use super::*;
use model::Layer;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub layers: Vec <Layer>,
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
			parse! (parser, params);
			let layers = parser.delim_fn ("\n", Parser::item).try_collect () ?;
			Ok (Self { layers, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for layer in self.layers.iter () { write! (formatter, "{}\n", layer) ?; }
		Ok (())
	}
}
