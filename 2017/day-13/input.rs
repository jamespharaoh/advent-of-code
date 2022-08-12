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
	pub fn parse (mut input: & [& str]) -> GenResult <Self> {
		let params = InputParams::parse (& mut input) ?;
		let layers = Parser::wrap_lines_auto_items (
			input.iter ().copied ().enumerate ()) ?;
		Ok (Self { layers, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for layer in self.layers.iter () { write! (formatter, "{}\n", layer) ?; }
		Ok (())
	}
}
