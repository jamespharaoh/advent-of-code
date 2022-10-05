#![ allow (clippy::missing_inline_in_public_items) ]

use super::*;

use input::Token;
use input::Tokens;

use digit::Digit;
pub use error::ParseError;
pub use number::Number;

pub type Val = u16;

mod digit {

	use super::*;

	#[ derive (Clone, Copy, Eq, PartialEq) ]
	pub struct Digit {
		depth: u8,
		value: u8,
	}

	impl Digit {
		pub const fn new (depth: u8, value: u8) -> Self {
			Self { depth, value }
		}
		pub const fn depth (self) -> u8 { self.depth }
		pub const fn value (self) -> u8 { self.value }
	}

	impl Debug for Digit {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter,
				"Digit {{ depth: {depth}, value: {value} }}",
				depth = self.depth (),
				value = self.value ())
		}
	}

}

mod error {

	use super::*;

	#[ derive (Clone, Copy, Debug) ]
	pub enum ParseError {
		Token,
		Mismatch,
		Number,
		UnexpectedEnd,
		UnexpectedOpen,
		UnexpectedClose,
		UnexpectedComma,
		UnexpectedValue,
	}

	impl Display for ParseError {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			let msg = match * self {
				Self::Token => "Error parsing token",
				Self::Mismatch => "Square brackets are not matched",
				Self::Number => "Number does not conform to rules",
				Self::UnexpectedEnd => "Unexpected end of tokens",
				Self::UnexpectedOpen => "Unexpected open bracket",
				Self::UnexpectedClose => "Unexpected close bracket",
				Self::UnexpectedComma => "Unexpected comma",
				Self::UnexpectedValue => "Unexpected value",
			};
			write! (formatter, "Error parsing Snailfish number: {msg}")
		}
	}

	impl Error for ParseError {
	}

}

mod number {

	use super::*;

	type DigitsVec = ArrayVec <Digit, 31>;

	#[ derive (Clone, Eq, PartialEq) ]
	pub struct Number {
		digits: DigitsVec,
	}

	impl Number {

		#[ must_use ]
		pub fn magnitude (& self) -> Val {
			let mut idx: usize = 0;
			let result = self.magnitude_real (& mut idx, 0);
			debug_assert_eq! (idx, self.digits.len ());
			result
		}

		fn magnitude_real (& self, idx: & mut usize, depth: u8) -> Val {
			let digit = self.digits [* idx];
			debug_assert! (depth <= digit.depth ());
			if digit.depth () == depth {
				* idx += 1;
				return Val::from_u8 (digit.value ()).unwrap ();
			}
			let left = self.magnitude_real (idx, depth + 1);
			let right = self.magnitude_real (idx, depth + 1);
			left * 3 + right * 2
		}

		pub fn sum <Iter> (iter: Iter) -> Option <Self>
				where Iter: IntoIterator <Item = Self> {
			iter.into_iter ().reduce (Self::add)
		}

		#[ allow (clippy::should_implement_trait) ]
		#[ must_use ]
		pub fn add (left: Self, right: Self) -> Self {
			Self::pair (left, right).reduce ()
		}

		#[ must_use ]
		pub fn pair (left: Self, right: Self) -> Self {
			let digits = iter::empty ()
				.chain (left.digits.iter ()
					.map (|digit| Digit::new (digit.depth () + 1, digit.value ())))
				.chain (right.digits.iter ()
					.map (|digit| Digit::new (digit.depth () + 1, digit.value ())))
				.collect ();
			Self { digits }
		}

		#[ must_use ]
		pub fn reduce (self) -> Self {

			// short circuit if no action is necessary

			if self.digits.iter ().all (|digit| digit.depth () < 5 && digit.value () < 10) {
				return self;
			}

			// set up state machine to switch between explode and split, and track range to work
			// on

			enum State { Explode, Split }
			let mut state = State::Explode;
			let mut start = 0;
			let mut end = self.digits.len ();
			let mut left = DigitsVec::new ();
			let mut right: DigitsVec = self.digits.into_iter ().rev ().collect ();

			// loop until there is no more work

			'OUTER: loop {
				match state {
					State::Explode => {

						// explode until `end`

						while left.len () < end && ! right.is_empty () {
							let digit = right.pop ().unwrap ();

							// if depth is less than five just copy this digit and continue

							if digit.depth () < 5 {
								if digit.value () < 10 && start == left.len () { start += 1; }
								left.push (digit);
								continue;
							}

							// explode this digit: first add the number to the left

							if ! left.is_empty () {
								let prev = left.pop ().unwrap ();
								if left.len () < start { start = left.len (); }
								let value = prev.value () + digit.value ();
								if value < 10 && start == left.len () { start += 1; }
								left.push (Digit::new (prev.depth (), value));
							}

							// then add the zero in the middle

							left.push (Digit::new (digit.depth () - 1, 0));

							// and finally add the number on the right

							let digit = right.pop ().unwrap ();
							if ! right.is_empty () {
								let next = right.pop ().unwrap ();
								let value = next.value () + digit.value ();
								right.push (Digit::new (next.depth (), value));
							}

							end -= 1;
						}

						// once complete, we switch over to split, starting from the first number
						// which is large enough to be split

						state = State::Split;
						while start < left.len () { right.push (left.pop ().unwrap ()); }

					},
					State::Split => {

						// split as much as we can

						while ! right.is_empty () {
							let digit = right.pop ().unwrap ();

							// digits less than ten can be copied over

							if digit.value () < 10 {
								left.push (digit);
								continue;
							}

							// split this digit, add to `right` because they may need exploding or
							// splitting again

							right.push (Digit::new (digit.depth () + 1, (digit.value () + 1) / 2));
							right.push (Digit::new (digit.depth () + 1, digit.value () / 2));

							// if we increased the depth past four then switch back to exploding,
							// but only for the pair we just added

							if digit.depth () == 4 {
								state = State::Explode;
								start = left.len ();
								end = left.len () + 2;
								continue 'OUTER;
							}

						}

						// if splitting reaches the end, then we are done

						break;

					},
				}
			}

			// `right` should be empty, return new digits in `left`

			debug_assert! (right.is_empty ());
			Self { digits: left }

		}

	}

	impl TryFrom <& Tokens> for Number {
		type Error = ParseError;
		fn try_from (tokens: & Tokens) -> Result <Self, ParseError> {
			fn take_one (
				digits: & mut DigitsVec,
				tokens: & mut SliceIter <Token>,
				depth: u8,
			) -> Result <(), ParseError> {
				if 5 < depth { return Err (ParseError::UnexpectedOpen) }
				match * tokens.next ().ok_or (ParseError::UnexpectedEnd) ? {
					Token::Open => (),
					Token::Close => return Err (ParseError::UnexpectedClose),
					Token::Comma => return Err (ParseError::UnexpectedComma),
					Token::Value (value) => {
						digits.push (Digit::new (depth, value));
						return Ok (());
					},
				}
				take_one (digits, tokens, depth + 1) ?;
				match * tokens.next ().ok_or (ParseError::UnexpectedEnd) ? {
					Token::Open => return Err (ParseError::UnexpectedOpen),
					Token::Close => return Err (ParseError::UnexpectedClose),
					Token::Comma => (),
					Token::Value (_) => return Err (ParseError::UnexpectedValue),
				}
				take_one (digits, tokens, depth + 1) ?;
				match * tokens.next ().ok_or (ParseError::UnexpectedEnd) ? {
					Token::Open => return Err (ParseError::UnexpectedOpen),
					Token::Close => (),
					Token::Comma => return Err (ParseError::UnexpectedComma),
					Token::Value (_) => return Err (ParseError::UnexpectedValue),
				}
				Ok (())
			}
			let mut digits = DigitsVec::new ();
			take_one (& mut digits, & mut tokens.tokens.iter (), 0) ?;
			Ok (Self { digits })
		}
	}

	impl FromStr for Number {
		type Err = ParseError;
		fn from_str (src: & str) -> Result <Self, ParseError> {
			let tokens = Tokens::parse_from_str (src)
				.map_err (|_err| ParseError::Token) ?;
			Self::try_from (& tokens)
		}
	}

	impl Display for Number {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			fn display_part (
				formatter: & mut fmt::Formatter,
				digits: & [Digit],
				idx: & mut usize,
				depth: u8,
			) -> fmt::Result {
				let digit = digits [* idx];
				debug_assert! (depth <= digit.depth ());
				if digit.depth () == depth {
					* idx += 1;
					return Display::fmt (& digit.value (), formatter);
				}
				formatter.write_char ('[') ?;
				display_part (formatter, digits, idx, depth + 1) ?;
				formatter.write_char (',') ?;
				display_part (formatter, digits, idx, depth + 1) ?;
				formatter.write_char (']') ?;
				Ok (())
			}
			display_part (formatter, & self.digits, & mut 0, 0)
		}
	}

	impl Debug for Number {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "Snaifish {{ digits: {:?} }}", self.digits) ?;
			Ok (())
		}
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn test_reduce () {
			assert_eq! (
				fish ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
				fish ("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").reduce ());
		}

		#[ test ]
		fn test_add () -> GenResult <()> {
			assert_eq! (
				fish ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
				Number::add (
					fish ("[[[[4,3],4],4],[7,[[8,4],9]]]"),
					fish ("[1,1]"),
				),
			);
			Ok (())
		}

		#[ test ]
		fn test_sum () {
			assert_eq! (None, Number::sum ([]));
			assert_eq! (
				Some (fish ("[[[[1,1],[2,2]],[3,3]],[4,4]]")),
				Number::sum (fishes (["[1,1]", "[2,2]", "[3,3]", "[4,4]"])));
			assert_eq! (
				Some (fish ("[[[[3,0],[5,3]],[4,4]],[5,5]]")),
				Number::sum (fishes (["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"])));
			assert_eq! (
				Some (fish ("[[[[5,0],[7,4]],[5,5]],[6,6]]")),
				Number::sum (fishes (["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"])));
			assert_eq! (
				Some (fish ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")),
				Number::sum (fishes ([
					"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
					"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
					"[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
					"[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
					"[7,[5,[[3,8],[1,4]]]]",
					"[[2,[2,2]],[8,[8,1]]]",
					"[2,9]",
					"[1,[[[9,3],9],[[9,0],[0,7]]]]",
					"[[[5,[7,4]],7],1]",
					"[[[[4,2],2],6],[8,7]]",
				])));
		}

		#[ test ]
		fn test_magnitude () {
			assert_eq! (143, fish ("[[1,2],[[3,4],5]]").magnitude ());
			assert_eq! (1384, fish ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude ());
			assert_eq! (445, fish ("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude ());
			assert_eq! (791, fish ("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude ());
			assert_eq! (1137, fish ("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude ());
			assert_eq! (3488, fish ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude ());
		}

		fn fish (src: & str) -> Number {
			Number::from_str (src).unwrap ()
		}

		fn fishes (items: impl IntoIterator <Item = & 'static str>) -> Vec <Number> {
			items.into_iter ()
				.map (fish)
				.collect ()
		}

	}

}
