use super::*;
use model::Particle;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub particles: Vec <Particle>,
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
		let particles = Parser::wrap_lines_auto_items (
			input.iter ().copied ().enumerate ()) ?;
		Ok (Self { particles, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for particle in self.particles.iter () {
			write! (formatter, "{}\n", particle) ?;
		}
		Ok (())
	}
}
