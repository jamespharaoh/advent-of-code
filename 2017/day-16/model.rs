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

	struct_parser_display! {
		Pos { id } = [ id = 0 .. 16 ]
	}

	impl Pos {

		#[ inline ]
		#[ must_use ]
		pub fn idx (self) -> usize {
			self.id.pan_usize ()
		}

	}

	impl LineItem for Pos {
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

	struct_parser_display! {
		Prog { id } = [
			@display { let id = (id + b'a').pan_char (); },
			id = 'a' ..= 'p',
			@parse { let id = id.pan_u8 () - b'a'; },
		]
	}

	impl Prog {

		#[ inline ]
		#[ must_use ]
		pub fn idx (self) -> usize {
			self.id.pan_usize ()
		}

	}

	impl LineItem for Prog {
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
