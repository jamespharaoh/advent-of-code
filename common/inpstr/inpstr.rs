use aoc_misc::prelude::*;

#[ derive (Clone) ]
pub enum InpStr <'inp> {
	Borrow (& 'inp str),
	RefCount (Rc <str>),
}

impl InpStr <'static> {

	#[ inline ]
	pub fn alloc (val: impl AsRef <str>) -> Self {
		Self::RefCount (Rc::from (val.as_ref ()))
	}

}

impl <'inp> InpStr <'inp> {

	#[ inline ]
	#[ must_use ]
	pub const fn borrow (val: & 'inp str) -> Self {
		Self::Borrow (val)
	}

	#[ inline ]
	#[ must_use ]
	pub fn to_owned (& self) -> String {
		self.deref ().to_owned ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn borrowed (& self) -> & 'inp str {
		match * self {
			Self::Borrow (val) => val,
			Self::RefCount (_) => panic! ("Can't call borrowed () on an InpStr::RefCount"),
		}
	}

	#[ inline ]
	#[ must_use ]
	pub fn as_str (& self) -> & str {
		match * self {
			Self::Borrow (val) => val,
			Self::RefCount (ref val) => val,
		}
	}

}

impl <'inp> Debug for InpStr <'inp> {
	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Debug::fmt (& ** self, formatter)
	}
}

impl <'inp> Deref for InpStr <'inp> {
	type Target = str;
	#[ inline ]
	fn deref (& self) -> & str {
		match * self {
			Self::Borrow (val) => val,
			Self::RefCount (ref val) => val,
		}
	}
}

impl <'inp> Display for InpStr <'inp> {
	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& ** self, formatter)
	}
}

impl <'inp> Eq for InpStr <'inp> {
}

impl <'inp> From <& 'inp str> for InpStr <'inp> {
	#[ inline ]
	fn from (from: & 'inp str) -> Self {
		Self::borrow (from)
	}
}

impl <'inp> Hash for InpStr <'inp> {
	#[ inline ]
	fn hash <Hshr: Hasher> (& self, hasher: & mut Hshr) {
		self.deref ().hash (hasher);
	}
}

impl <'inp> Ord for InpStr <'inp> {
	#[ inline ]
	fn cmp (& self, other: & Self) -> Ordering {
		self.deref ().cmp (& ** other)
	}
}

impl <'inp> PartialEq for InpStr <'inp> {
	#[ inline ]
	fn eq (& self, other: & Self) -> bool {
		self.deref ().eq (& ** other)
	}
}

impl <'inp> PartialEq <& str> for InpStr <'inp> {
	#[ inline ]
	fn eq (& self, other: && str) -> bool {
		self.deref ().eq (* other)
	}
}

impl <'inp> PartialEq <& str> for & InpStr <'inp> {
	#[ inline ]
	fn eq (& self, other: && str) -> bool {
		(* self).deref ().eq (* other)
	}
}

impl <'inp> PartialEq <InpStr <'inp>> for & str {
	#[ inline ]
	fn eq (& self, other: & InpStr <'inp>) -> bool {
		self.eq (&& ** other)
	}
}

impl <'inp> PartialOrd for InpStr <'inp> {
	#[ inline ]
	fn partial_cmp (& self, other: & Self) -> Option <Ordering> {
		self.deref ().partial_cmp (& ** other)
	}
}

// conversions to other types

impl <'inp> From <InpStr <'inp>> for Rc <str> {
	#[ inline ]
	fn from (val: InpStr <'inp>) -> Self {
		match val {
			InpStr::Borrow (val) => Self::from (val),
			InpStr::RefCount (val) => val,
		}
	}
}

impl <'inp> From <InpStr <'inp>> for String {
	#[ inline ]
	fn from (val: InpStr <'inp>) -> Self {
		val.to_owned ()
	}
}
