use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub deps: Vec <(char, char)>,
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
		let parse_dep = |parser: & mut Parser| {
			parse! (parser,
				"Step ", before = 'A' ..= 'Z', " must be finished before step ",
				after = 'A' ..= 'Z', " can begin.");
			Ok ((before, after))
		};
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params, @lines deps = parse_dep);
			Ok (Self { deps, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for & (before, after) in self.deps.iter () {
			write! (formatter,
				"Step {before} must be finished before step {after} can begin.\n") ?;
		}
		Ok (())
	}
}
