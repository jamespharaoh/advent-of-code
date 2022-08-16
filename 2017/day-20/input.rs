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
	pub fn parse (input: & [& str]) -> GenResult <Self> {
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params, (@line_items particles));
			Ok (Self { particles, params })
		})
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
