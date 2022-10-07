use super::*;

#[ derive (Clone, Eq, Hash, PartialEq) ]
pub struct State (Vec <u8>);

impl State {
	pub fn parse (input: & str) -> GenResult <Self> {
		input.chars ()
			.map (|ch| Ok (ch.to_digit (10).ok_or ("Invalid input") ?.pan_u8 ()))
			.collect::<GenResult <_>> ()
	}
	pub fn iter (& self) -> SliceIter <'_, u8> {
		self.0.iter ()
	}
}

impl Borrow <[u8]> for State {
	fn borrow (& self) -> & [u8] {
		self.0.as_slice ()
	}
}

impl Deref for State {
	type Target = Vec <u8>;
	fn deref (& self) -> & Vec <u8> {
		& self.0
	}
}

impl FromIterator <u8> for State {
	fn from_iter <IntoIter> (iter: IntoIter) -> Self
			where IntoIter: IntoIterator <Item = u8> {
		Self (iter.into_iter ().collect ())
	}
}

impl TryFrom <Vec <u8>> for State {
	type Error = GenError;
	fn try_from (nums: Vec <u8>) -> GenResult <Self> {
		if nums.iter ().copied ().any (|num| (1 ..= 9).contains (& num)) {
			Err ("Digits must be 1-9") ?;
		}
		Ok (Self (nums))
	}
}

impl Debug for State {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "State (len={}, {})", self.0.len (), self) ?;
		Ok (())
	}
}

impl Display for State {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for & val in self.0.iter () {
			write! (formatter, "{}", char::from_digit (val.pan_u32 (), 10).unwrap ()) ?;
		}
		Ok (())
	}
}

impl PartialOrd for State {
	fn partial_cmp (& self, other: & Self) -> Option <Ordering> {
		Some (Ord::cmp (self, other))
	}
}

impl Ord for State {
	fn cmp (& self, other: & Self) -> Ordering {
		Ord::cmp (& self.0.len (), & other.0.len ())
			.then (Ord::cmp (& self.0, & other.0))
	}
}
