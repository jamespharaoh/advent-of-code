use super::*;

pub type Val = u8;

enum_decl_parser_display! {
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub enum Item {
		List (items: Vec <Item>) = [ "[", @delim "," items, "]" ],
		Value (value: Val) = [ value ],
	}
}

impl Item {

	fn value (& self) -> Option <Val> {
		if let & Self::Value (value) = self { Some (value) } else { None }
	}

	fn items (& self) -> Cow <'_, [Self]> {
		match * self {
			Self::List (ref items) => Cow::Borrowed (items),
			Self::Value (value) => Cow::Owned (vec! [ Self::Value (value) ]),
		}
	}

}

impl Ord for Item {

	fn cmp (& self, other: & Self) -> Ordering {
		if let (Some (left), Some (right)) = (self.value (), other.value ()) {
			left.cmp (& right)
		} else {
			self.items ().cmp (& other.items ())
		}
	}

}

impl PartialOrd for Item {

	fn partial_cmp (& self, other: & Self) -> Option <Ordering> {
		Some (self.cmp (other))
	}

}
