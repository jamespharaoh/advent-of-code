use super::*;

pub use line::Line;
pub use line::LineItem;
pub use pos::Pos;
pub use prog::Prog;
pub use step::Step;

mod pos {

	use super::*;

	#[ derive (Clone, Copy, Debug) ]
	pub struct Pos { id: u8 }

	impl Pos {

		#[ inline ]
		#[ must_use ]
		pub fn idx (self) -> usize {
			self.id.pan_usize ()
		}

	}

	impl Display for Pos {

		#[ inline ]
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "{}", self.id) ?;
			Ok (())
		}

	}

	impl LineItem for Pos {
	}

	impl <'inp> FromParser <'inp> for Pos {

		#[ inline ]
		fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
			let id = parser.uint () ?;
			if ! (0 ..= 15).contains (& id) { return Err (parser.err ()) }
			Ok (Self { id })
		}

	}

	impl TryFrom <u8> for Pos {

		type Error = ();

		#[ inline ]
		fn try_from (id: u8) -> Result <Self, ()> {
			if ! (0 .. 16).contains (& id) { return Err (()) }
			Ok (Self { id })
		}

	}

}

mod prog {

	use super::*;

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub struct Prog { id: u8 }

	impl Prog {

		#[ inline ]
		#[ must_use ]
		pub fn idx (self) -> usize {
			self.id.pan_usize ()
		}

		#[ inline ]
		#[ must_use ]
		pub fn as_char (self) -> char {
			match self.id {
				0x00 => 'a', 0x01 => 'b', 0x02 => 'c', 0x03 => 'd',
				0x04 => 'e', 0x05 => 'f', 0x06 => 'g', 0x07 => 'h',
				0x08 => 'i', 0x09 => 'j', 0x0a => 'k', 0x0b => 'l',
				0x0c => 'm', 0x0d => 'n', 0x0e => 'o', 0x0f => 'p',
				_ => unreachable! (),
			}
		}

	}

	impl Display for Prog {

		#[ inline ]
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			formatter.write_char (self.as_char ()) ?;
			Ok (())
		}

	}

	impl <'inp> FromParser <'inp> for Prog {

		fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
			Self::try_from (parser.expect_next () ?).map_err (|_err| parser.err ())
		}

	}

	impl LineItem for Prog {
	}

	impl TryFrom <char> for Prog {

		type Error = ();

		#[ inline ]
		fn try_from (ch: char) -> Result <Self, ()> {
			let id = match ch {
				'a' => 0x00, 'b' => 0x01, 'c' => 0x02, 'd' => 0x03,
				'e' => 0x04, 'f' => 0x05, 'g' => 0x06, 'h' => 0x07,
				'i' => 0x08, 'j' => 0x09, 'k' => 0x0a, 'l' => 0x0b,
				'm' => 0x0c, 'n' => 0x0d, 'o' => 0x0e, 'p' => 0x0f,
				_ => return Err (()),
			};
			Ok (Self { id })
		}

	}

	impl TryFrom <u8> for Prog {

		type Error = ();

		#[ inline ]
		fn try_from (id: u8) -> Result <Self, ()> {
			if ! (0 .. 16).contains (& id) { return Err (()) }
			Ok (Self { id })
		}

	}

}

mod line {

	use super::*;

	pub trait LineItem: Copy + Display + TryFrom <u8> {
	}

	#[ derive (Copy, Clone, Debug) ]
	pub struct Line <Item: LineItem> {
		progs: [Item; 16],
	}

	impl <Item: LineItem> Default for Line <Item> {

		#[ inline ]
		fn default () -> Self {
			let progs = array::from_fn (|idx|
				Item::try_from (idx.pan_u8 ()).map_err (|_err| unreachable! ()).unwrap ());
			Self { progs }
		}

	}

	impl <Item: LineItem> Deref for Line <Item> {

		type Target = [Item; 16];

		#[ inline ]
		fn deref (& self) -> & [Item; 16] {
			& self.progs
		}

	}

	impl <Item: LineItem> DerefMut for Line <Item> {

		#[ inline ]
		fn deref_mut (& mut self) -> & mut [Item; 16] {
			& mut self.progs
		}

	}

	impl <Item: LineItem> Display for Line <Item> {

		#[ inline ]
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			for prog in self.progs {
				Display::fmt (& prog, formatter) ?;
			}
			Ok (())
		}

	}

	impl <Item: LineItem> From <[Item; 16]> for Line <Item> {

		#[ inline ]
		fn from (progs: [Item; 16]) -> Self {
			Self { progs }
		}

	}

}

mod step {

	use super::*;

	enum_decl_parser_display! {
		#[ derive (Copy, Clone, Debug) ]
		pub enum Step {
			Spin (pos: Pos) = [ "s", pos ],
			Exchange (pos_0: Pos, pos_1: Pos) = [ "x", pos_0, "/", pos_1 ],
			Partner (prg_0: Prog, prg_1: Prog) = [ "p", prg_0, "/", prg_1 ],
		}
	}

}
