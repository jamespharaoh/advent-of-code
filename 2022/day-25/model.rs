use super::*;

pub type Val = i64;

#[ derive (Clone, Debug) ]
pub struct Snafu {
	digits: Vec <SnafuDigit>,
}

struct_parser_display! {
	Snafu { digits } = [ @collect digits ]
}

impl Snafu {

	pub fn to_val (& self) -> GenResult <Val> {
		let num =
			self.digits.iter ().copied ()
				.try_fold (0, |sum: Val, digit: SnafuDigit| {
					chk! (sum * 5 + digit.to_val ())
				}) ?;
		if num < Val::ZERO {
			return Err ("Snafu number must not be negative".into ());
		}
		Ok (num)
	}

	pub fn from_val (mut val: Val) -> Self {
		assert! (Val::ZERO <= val);
		if val == Val::ZERO { return Self { digits: vec! [ SnafuDigit::Zero ] } }
		let mut digits = Vec::new ();
		while val != 0 {
			let (digit, digit_val) = match val.rem_euclid (5) {
				0 => (SnafuDigit::Zero, 0),
				1 => (SnafuDigit::PlusOne, 1),
				2 => (SnafuDigit::PlusTwo, 2),
				3 => (SnafuDigit::MinusTwo, -2),
				4 => (SnafuDigit::MinusOne, -1),
				_ => unreachable! (),
			};
			digits.push (digit);
			val -= digit_val;
			val /= 5;
		}
		digits.reverse ();
		Self { digits }
	}

}

#[ test ]
fn snafu_parse_test () {
	assert_eq_ok! (10, Snafu::parse_from_str ("20").unwrap ().to_val ());
	assert_eq_ok! (8, Snafu::parse_from_str ("2=").unwrap ().to_val ());
	assert_eq_ok! (976, Snafu::parse_from_str ("2=-01").unwrap ().to_val ());
}

#[ test ]
fn snafu_display_test () {
	assert_eq! ("20", Snafu::from_val (10).to_string ().as_str ());
	assert_eq! ("2=", Snafu::from_val (8).to_string ().as_str ());
	assert_eq! ("2=-01", Snafu::from_val (976).to_string ().as_str ());
}

enum_decl_parser_display! {

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum SnafuDigit {
		MinusTwo = [ "=" ],
		MinusOne = [ "-" ],
		Zero = [ "0" ],
		PlusOne = [ "1" ],
		PlusTwo = [ "2" ],
	}

}

impl SnafuDigit {

	fn to_val (self) -> Val {
		match self {
			Self::MinusTwo => -2,
			Self::MinusOne => -1,
			Self::Zero => 0,
			Self::PlusOne => 1,
			Self::PlusTwo => 2,
		}
	}

}
