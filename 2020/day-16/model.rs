use super::*;

pub type Val = u16;

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Field <'inp> {
	pub name: InpStr <'inp>,
	pub first: (Val, Val),
	pub second: (Val, Val),
}

struct_parser_display! {
	input_lifetime = 'inp;
	Field <'inp> { name, first: (first_min, first_max), second: (second_min, second_max) } = [
		@str name = (|ch| { ch.is_ascii_lowercase () || ch == ' ' }, 1 .. ), ": ",
		first_min, "-", first_max, " or ", second_min, "-", second_max,
	]
}

impl <'inp> Field <'inp> {
	#[ must_use ]
	pub fn contains (& self, val: Val) -> bool {
		(self.first.0 ..= self.first.1).contains (& val)
			|| (self.second.0 ..= self.second.1).contains (& val)
	}
}

wrapper_deref_mut! {
	#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub struct Ticket {
		pub fields: Vec <Val>,
	}
}

struct_parser_display! {
	Ticket { fields } = [ @delim "," fields ]
}
