use super::*;

#[ derive (Clone, Copy, Debug) ]
pub struct Display {
	pub digits: [Digit; 10],
	pub value: [Digit; 4],
}

struct_parser_display! {
	Display { digits, value } = [ @array_delim " " digits, " | ", @array_delim " " value ]
}

#[ derive (Clone, Copy, Debug) ]
pub struct Digit {
	pub segments: u8,
}

impl Digit {

	#[ inline ]
	#[ must_use ]
	pub const fn on (self) -> u8 {
		self.segments
	}

	#[ inline ]
	#[ must_use ]
	pub const fn off (self) -> u8 {
		self.segments ^ 0x7f
	}

	#[ inline ]
	#[ must_use ]
	pub const fn num_segments (self) -> u32 {
		self.segments.count_ones ()
	}

}

impl <'inp> FromParser <'inp> for Digit {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		let mut segments = 0_u8;
		let err = parser.err ();
		let rest = parser.take_rest_while (|ch| ('a' ..= 'g').contains (& ch), 2 ..= 7) ?;
		for ch in rest.chars () {
			let bit = 1 << (ch.pan_u32 () - 'a'.pan_u32 ());
			if segments & bit != 0 { return Err (err) }
			segments |= bit;
		}
		Ok (Self { segments })
	}
}

impl fmt::Display for Digit {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		let mut segments = self.segments;
		let mut ch = 'a';
		for _ in 0_u32 .. 7 {
			if segments & 1 != 0 { formatter.write_char (ch) ?; }
			segments >>= 1_u32;
			ch = (ch.pan_u32 () + 1).pan_char ();
		}
		Ok (())
	}
}
