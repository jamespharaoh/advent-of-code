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
	pub fn parse (mut input: & [& str]) -> GenResult <Self> {
		let params = InputParams::parse (& mut input) ?;
		if input.len () != 1 { return Err ("Input must be exactly one line".into ()) }
		let steps = Parser::wrap_auto (input [0], |parser| {
			let steps = parser.delim_fn (",", Parser::item).collect::<ParseResult <_>> () ?;
			parser.end () ?;
			Ok (steps)
		}) ?;
		Ok (Self { steps, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for step in self.steps.iter () {
			Display::fmt (step, formatter) ?;
		}
		Ok (())
	}
}
