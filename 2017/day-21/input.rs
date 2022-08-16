use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub rules: Vec <InputRule>,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub iters_one: u32 = ("ITERS_ONE=", 5, 1_u32 ..),
		pub iters_two: u32 = ("ITERS_TWO=", 18, 1_u32 ..),
		pub check_rules: bool = ("CHECK_RULES=", true, false ..= true),
	}
}

impl Input {
	pub fn parse (input: & [& str]) -> GenResult <Self> {
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params, (@line_items rules));
			Ok (Self { rules, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for rule in self.rules.iter () {
			write! (formatter, "{}\n", rule) ?;
		}
		Ok (())
	}
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum InputRule {
	TwoToThree (u8, u16),
	ThreeToFour (u16, u16),
}

impl Display for InputRule {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::TwoToThree (from, to) => write! (formatter, "{} => {}\n", from, to) ?,
			Self::ThreeToFour (from, to) => write! (formatter, "{} => {}\n", from, to) ?,
		}
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for InputRule {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parser.any ()
			.of (|parser| {
				let from = parse_pixels (parser, 2) ?;
				parse! (parser, (@confirm), " => ");
				let to = parse_pixels (parser, 3) ?;
				Ok (Self::TwoToThree (from.as_u8 (), to.as_u16 ()))
			})
			.of (|parser| {
				let from = parse_pixels (parser, 3) ?;
				parse! (parser, (@confirm), " => ");
				let to = parse_pixels (parser, 4) ?;
				Ok (Self::ThreeToFour (from.as_u16 (), to.as_u16 ()))
			})
			.done ()
	}
}

pub fn parse_pixels (parser: & mut Parser <'_>, size: usize) -> ParseResult <u64> {
	let mut val = 0_u64;
	let new_bit = (1_u64 << ((size * size) - 1)).as_u64 ();
	for row in 0 .. size {
		if row > 0 { parse! (parser, "/"); }
		for _col in 0 .. size {
			parse! (parser, (bit: InputPixel));
			val >>= 1_u32;
			if bit == InputPixel::On { val |= new_bit; }
		}
	}
	Ok (val)
}

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum InputPixel { Off = ".", On = "#" }
}
