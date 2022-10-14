use super::*;
use model::Village;

pub const MAX_PIPES: usize = 8;

pub type InputPipes = Vec <InputPipe>;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub pipes: InputPipes,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}

#[ derive (Clone, Debug) ]
pub struct InputPipe {
	pub left: Village,
	pub right: ArrayVec <Village, MAX_PIPES>,
}

impl Input {
	pub fn parse (input: & [& str]) -> GenResult <Self> {
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params);
			let pipes = parser.delim_fn ("\n", Parser::item).collect ();
			Ok (Self { pipes, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for pipe in self.pipes.iter () { write! (formatter, "{}\n", pipe) ?; }
		Ok (())
	}
}

impl Display for InputPipe {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "{} <-> ", self.left) ?;
		for right in Itertools::intersperse (
				self.right.iter ().copied ().map (Either::Left),
				Either::Right (", ")) {
			write! (formatter, "{}", right) ?;
		}
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for InputPipe {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, left, " <-> ");
		let right_temp: ArrayVec <Village, 11> = parser
			.delim_fn (", ", Parser::uint)
			.take (MAX_PIPES + 1)
			.collect ();
		if right_temp.len () > MAX_PIPES { return Err (parser.err ()) }
		let right = right_temp.into_iter ().collect ();
		Ok (Self { left, right })
	}
}
