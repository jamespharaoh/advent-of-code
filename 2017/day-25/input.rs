use super::*;
use model::State;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub begin_state: char,
	pub num_steps: u32,
	pub states: Vec <State>,
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
			let parse_state_id = |parser: & mut Parser| {
				match parser.next () {
					Some (ch @ 'A' ..= 'Z') => Ok (ch),
					_ => Err (parser.err ()),
				}
			};
			parse! (parser,
				"Begin in state ", (begin_state = parse_state_id), ".\n",
				"Perform a diagnostic checksum after ", num_steps, " steps.\n",
				"\n");
			let states = parser.delim_fn ("\n\n", Parser::item).try_collect () ?;
			Ok (Self { begin_state, num_steps, states, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter,
			concat! (
				"Begin in state {begin_state}.\n",
				"Perform a diagnostic checksum after {num_steps} steps.\n",
			),
			begin_state = self.begin_state,
			num_steps = self.num_steps,
		) ?;
		for state in self.states.iter () {
			write! (formatter, "\n") ?;
			Display::fmt (state, formatter) ?;
		}
		Ok (())
	}
}
