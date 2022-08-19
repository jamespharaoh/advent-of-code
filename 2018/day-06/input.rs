use super::*;
use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub posns: Vec <Pos>,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub dist_two: u32 = ("DIST_TWO=", 10_000_u32, (1_u32 ..= 100_000)),
	}
}

impl Input {
	pub fn parse (input: & [& str]) -> GenResult <Self> {
		let parse_pos = |parser: & mut Parser| {
			parse! (parser, pos_x, ", ", pos_y);
			Ok (Pos { y: pos_y, x: pos_x })
		};
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params, @lines posns = parse_pos);
			Ok (Self { posns, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for pos in self.posns.iter () {
			write! (formatter, "{}, {}\n", pos.x, pos.y) ?;
		}
		Ok (())
	}
}
