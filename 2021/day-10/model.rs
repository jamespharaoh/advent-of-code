use super::*;

use Delim::{ Round, Square, Curly, Angle };
use Mode::{ Open, Close };
use Token::{
	RoundOpen, RoundClose, SquareOpen, SquareClose,
	CurlyOpen, CurlyClose, AngleOpen, AngleClose,
};

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Token {
		RoundOpen = "(",
		RoundClose = ")",
		SquareOpen = "[",
		SquareClose = "]",
		CurlyOpen = "{",
		CurlyClose = "}",
		AngleOpen = "<",
		AngleClose = ">",
	}
}

impl Token {

	#[ must_use ]
	pub const fn delim (self) -> Delim {
		match self {
			RoundOpen | RoundClose => Round,
			SquareOpen | SquareClose => Square,
			CurlyOpen | CurlyClose => Curly,
			AngleOpen | AngleClose => Angle,
		}
	}

	#[ must_use ]
	pub const fn mode (self) -> Mode {
		match self {
			RoundOpen | SquareOpen | CurlyOpen | AngleOpen => Open,
			RoundClose | SquareClose | CurlyClose | AngleClose => Close,
		}
	}

}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum Delim { Round, Square, Curly, Angle }

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum Mode { Open, Close }

impl Delim {

	#[ must_use ]
	pub const fn mismatched_points (self) -> u64 {
		match self {
			Self::Round => 3,
			Self::Square => 57,
			Self::Curly => 1197,
			Self::Angle => 25137,
		}
	}

	#[ must_use ]
	pub const fn not_closed_points (self) -> u64 {
		match self {
			Self::Round => 1,
			Self::Square => 2,
			Self::Curly => 3,
			Self::Angle => 4,
		}
	}

}
