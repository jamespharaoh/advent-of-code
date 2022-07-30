//! Advent of Code 2021: Day 18: Snailfish
//!
//! [https://adventofcode.com/2021/day/18](https://adventofcode.com/2021/day/18)

use aoc_common::*;

puzzle_info! {
	name = "Snailfish";
	year = 2021;
	day = 18;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;
	use snailfish::Snailfish;

	pub fn part_one (lines: & [& str]) -> GenResult <i64> {
		let sum = Snailfish::sum (lines.iter ().copied ().map (Snailfish::parse));
		let magnitude = sum.magnitude ();
		Ok (magnitude)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <i64> {
		let numbers: Vec <Snailfish> = lines.iter ().copied ().map (Snailfish::parse).collect ();
		let mut best: i64 = i64::MIN;
		for i in 0 .. numbers.len () {
			for j in 0 .. numbers.len () {
				if i == j { continue }
				let value = Snailfish::add (numbers [i].clone (), numbers [j].clone ()).magnitude ();
				if value > best { best = value; }
			}
		}
		Ok (best)
	}

}

mod snailfish {

	use super::*;
	use model::Token;
	use model::Tokens;

	#[ derive (Clone, Eq, PartialEq) ]
	pub struct Snailfish {
		tokens: Rc <Tokens>,
	}

	impl Snailfish {

		pub fn magnitude (& self) -> i64 {
			let mut pos: usize = 0;
			let result = self.magnitude_real (& mut pos);
			if pos != self.tokens.len () { panic! () }
			result
		}

		fn magnitude_real (& self, pos: & mut usize) -> i64 {
			if * pos + 1 > self.tokens.len () { panic! () }
			if self.tokens [* pos].is_value () {
				let value = self.tokens [* pos].value ();
				* pos += 1;
				return value;
			}
			if ! self.tokens [* pos].is_open () { panic! () }
			* pos += 1;
			let left = self.magnitude_real (pos);
			if ! self.tokens [* pos].is_comma () { panic! () }
			* pos += 1;
			let right = self.magnitude_real (pos);
			if ! self.tokens [* pos].is_close () { panic! () }
			* pos += 1;
			left * 3 + right * 2
		}

		pub fn sum <Item: Borrow <Snailfish>, Iter: Iterator <Item = Item>> (iter: Iter) -> Snailfish {
			let mut sum = None;
			for item in iter {
				let item = item.borrow ();
				if let Some (prev_sum) = sum {
					sum = Some (Snailfish::add (prev_sum, item));
				} else {
					sum = Some (item.to_owned ());
				}
			}
			sum.unwrap ()
		}

		pub fn add (left: impl Into <Snailfish>, right: impl Into <Snailfish>) -> Snailfish {
			Snailfish::pair (left, right).reduce ()
		}

		pub fn pair (left: impl Into <Snailfish>, right: impl Into <Snailfish>) -> Snailfish {
			let Snailfish { tokens: left_tokens } = left.into ();
			let Snailfish { tokens: right_tokens } = right.into ();
			match Rc::try_unwrap (left_tokens) {
				Ok (mut left_tokens) => {
					left_tokens.insert (0, Token::Open);
					left_tokens.extend (
						iter::once (Token::Comma)
							.chain (right_tokens.iter ().copied ())
							.chain (iter::once (Token::Close)));
					Snailfish { tokens: left_tokens.into () }
				},
				Err (left_tokens) => match Rc::try_unwrap (right_tokens) {
					Ok (mut right_tokens) => {
						right_tokens.splice (0 .. 0,
							iter::once (Token::Open)
								.chain (left_tokens.iter ().copied ())
								.chain (iter::once (Token::Comma)));
						right_tokens.push (Token::Close);
						Snailfish { tokens: right_tokens.into () }
					},
					Err (right_tokens) => {
						let tokens =
							iter::once (Token::Open)
								.chain (left_tokens.iter ().copied ())
								.chain (iter::once (Token::Comma))
								.chain (right_tokens.iter ().copied ())
								.chain (iter::once (Token::Close))
								.collect::<Tokens> ();
						Snailfish { tokens: tokens.into () }
					},
				},
			}
		}

		pub fn reduce (self) -> Snailfish {
			let mut val = self;
			loop {
				let reduced;
				(reduced, val) = val.reduce_once ();
				if ! reduced { return val }
			}
		}

		pub fn reduce_once (self) -> (bool, Snailfish) {
			let mut depth = 0;
			for (pos, token) in self.tokens.iter ().enumerate () {
				match token {
					Token::Open => {
						if depth == 4 {
							return (true, self.reduce_explode (pos));
						} else {
							depth += 1;
						}
					},
					Token::Close => {
						if depth == 0 { panic! () }
						depth -= 1;
					},
					_ => (),
				}
			}
			for (pos, token) in self.tokens.iter ().enumerate () {
				if let & Token::Value (value) = token {
					if value >= 10 {
						return (true, self.reduce_split (pos));
					}
				}
			}
			(false, self)
		}

		fn reduce_explode (self, pos: usize) -> Snailfish {
			if ! (self.tokens [pos].is_open ()
					&& self.tokens [pos + 1].is_value ()
					&& self.tokens [pos + 2].is_comma ()
					&& self.tokens [pos + 3].is_value ()
					&& self.tokens [pos + 4].is_close ()) {
				panic! ();
			}
			let explode_left = self.tokens [pos + 1].value ();
			let explode_right = self.tokens [pos + 3].value ();
			match Rc::try_unwrap (self.tokens) {
				Ok (mut tokens) => {
					if let Some (pos_left) =
							tokens [0 .. pos].iter ().rposition (Token::is_value) {
						tokens [pos_left] =
							Token::Value (tokens [pos_left].value () + explode_left);
					}
					tokens.splice (pos .. pos + 5, iter::once (Token::Value (0)));
					if let Some (pos_right) =
							tokens [pos + 1 ..].iter ().position (Token::is_value) {
						tokens [pos + 1 + pos_right] =
							Token::Value (tokens [pos + 1 + pos_right].value () + explode_right);
					}
					Snailfish { tokens: tokens.into () }
				},
				Err (tokens) => {
					let mut result = Tokens::new ();
					if let Some (pos_left) = tokens [0 .. pos].iter ().rposition (Token::is_value) {
						result.extend (tokens [0 .. pos_left].iter ().copied ());
						result.push (Token::Value (tokens [pos_left].value () + explode_left));
						result.extend (tokens [pos_left + 1 .. pos].iter ().copied ());
					} else {
						result.extend (tokens [0 .. pos].iter ().copied ());
					}
					result.push (Token::Value (0));
					if let Some (pos_right) = tokens [pos + 5 ..].iter ().position (Token::is_value) {
						let pos_right = pos + 5 + pos_right;
						result.extend (tokens [pos + 5 .. pos_right].iter ().copied ());
						result.push (Token::Value (tokens [pos_right].value () + explode_right));
						result.extend (tokens [pos_right + 1 ..].iter ().copied ());
					} else {
						result.extend (tokens [pos + 5 ..].iter ().copied ());
					}
					Snailfish { tokens: result.into () }
				},
			}
		}

		fn reduce_split (self, pos: usize) -> Snailfish {
			let value = self.tokens [pos].value ();
			match Rc::try_unwrap (self.tokens) {
				Ok (mut tokens) => {
					tokens.splice (pos .. pos + 1, [
						Token::Open,
						Token::Value (value / 2),
						Token::Comma,
						Token::Value ((value + 1) / 2),
						Token::Close,
					]);
					Snailfish { tokens: tokens.into () }
				},
				Err (tokens) => {
					let mut result = Tokens::new ();
					result.extend (tokens [0 .. pos].iter ().copied ());
					result.push (Token::Open);
					result.push (Token::Value (value / 2));
					result.push (Token::Comma);
					result.push (Token::Value ((value + 1) / 2));
					result.push (Token::Close);
					result.extend (tokens [pos + 1 ..].iter ().copied ());
					Snailfish { tokens: result.into () }
				},
			}
		}

		pub fn parse (input: & str) -> Snailfish {
			Snailfish {
				tokens: model::parse_tokens (& mut input.chars ().peekable ()).into (),
			}
		}

	}

	impl From <& Snailfish> for Snailfish {
		fn from (val: & Snailfish) -> Snailfish {
			Snailfish { tokens: Rc::clone (& val.tokens) }
		}
	}

	impl Display for Snailfish {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			for token in self.tokens.iter () {
				match token {
					Token::Open => write! (formatter, "[") ?,
					Token::Close => write! (formatter, "]") ?,
					Token::Comma => write! (formatter, ",") ?,
					Token::Value (value) => write! (formatter, "{}", value) ?,
				}
			}
			Ok (())
		}
	}

	impl Debug for Snailfish {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "Snaifish \"{}\"", self) ?;
			Ok (())
		}
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn test_reduce_once_explode () {
			assert_eq! (
				(true, Snailfish::parse ("[[[[0,9],2],3],4]")),
		        Snailfish::parse ("[[[[[9,8],1],2],3],4]").reduce_once (),
			);
			assert_eq! (
				(true, Snailfish::parse ("[7,[6,[5,[7,0]]]]")),
				Snailfish::parse ("[7,[6,[5,[4,[3,2]]]]]").reduce_once (),
			);
			assert_eq! (
				(true, Snailfish::parse ("[[6,[5,[7,0]]],3]")),
				Snailfish::parse ("[[6,[5,[4,[3,2]]]],1]").reduce_once (),
			);
			assert_eq! (
				(true, Snailfish::parse ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")),
			    Snailfish::parse ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").reduce_once (),
			);
			assert_eq! (
				(true, Snailfish::parse ("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")),
				Snailfish::parse ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").reduce_once (),
			);
		}

		#[ test ]
		fn test_reduce_once_split () {
			assert_eq! (
				(true, Snailfish::parse ("[5,5]")),
				Snailfish::parse ("10").reduce_once (),
			);
			assert_eq! (
				(true, Snailfish::parse ("[5,6]")),
				Snailfish::parse ("11").reduce_once (),
			);
			assert_eq! (
				(true, Snailfish::parse ("[6,6]")),
				Snailfish::parse ("12").reduce_once (),
			);
		}

		#[ test ]
		fn test_add () {
			assert_eq! (
				Snailfish::parse ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
				Snailfish::add (
					Snailfish::parse ("[[[[4,3],4],4],[7,[[8,4],9]]]"),
					Snailfish::parse ("[1,1]"),
				),
			);
		}

		#[ test ]
		fn test_sum () {
			assert_eq! (
				Snailfish::parse ("[[[[1,1],[2,2]],[3,3]],[4,4]]"),
				Snailfish::sum (vec! ["[1,1]", "[2,2]", "[3,3]", "[4,4]"].into_iter ().map (Snailfish::parse)),
			);
			assert_eq! (
				Snailfish::parse ("[[[[3,0],[5,3]],[4,4]],[5,5]]"),
				Snailfish::sum (vec! ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"].into_iter ().map (Snailfish::parse)),
			);
			assert_eq! (
				Snailfish::parse ("[[[[5,0],[7,4]],[5,5]],[6,6]]"),
				Snailfish::sum (vec! ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"].into_iter ().map (Snailfish::parse)),
			);
			assert_eq! (
				Snailfish::parse ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
				Snailfish::sum (vec! [
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
				].into_iter ().map (Snailfish::parse)),
			);
		}

		#[ test ]
		fn test_magnitude () {
			assert_eq! (143, Snailfish::parse ("[[1,2],[[3,4],5]]").magnitude ());
			assert_eq! (1384, Snailfish::parse ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude ());
			assert_eq! (445, Snailfish::parse ("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude ());
			assert_eq! (791, Snailfish::parse ("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude ());
			assert_eq! (1137, Snailfish::parse ("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude ());
			assert_eq! (3488, Snailfish::parse ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude ());
		}

	}

}

mod model {

	use super::*;

	//pub type Tokens = ArrayVec <Token, 125>;
	pub type Tokens = Vec <Token>;

	pub fn parse_tokens (input_iter: & mut Peekable <Chars <'_>>) -> Tokens {
		let mut result = Tokens::new ();
		while let Some (letter) = input_iter.peek () {
			match letter {
				'[' => { input_iter.next ().unwrap (); result.push (Token::Open); },
				']' => { input_iter.next ().unwrap (); result.push (Token::Close); },
				',' => { input_iter.next ().unwrap (); result.push (Token::Comma); },
				'0' ..= '9' => {
					let mut value_buf = String::new ();
					loop {
						if let Some (letter) = input_iter.peek () {
							if ('0' ..= '9').contains (letter) {
								value_buf.push (input_iter.next ().unwrap ());
								continue;
							}
						}
						break;
					}
					result.push (Token::Value (value_buf.parse ().unwrap ()));
				},
				_ => panic! (),
			}
		}
		result
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Token {
		Open,
		Close,
		Comma,
		Value (i64),
	}

	impl Token {
		pub fn is_open (& self) -> bool { matches! (* self, Token::Open) }
		pub fn is_close (& self) -> bool { matches! (* self, Token::Close) }
		pub fn is_comma (& self) -> bool { matches! (* self, Token::Comma) }
		pub fn is_value (& self) -> bool { matches! (* self, Token::Value (_)) }
		pub fn value (& self) -> i64 {
			match * self {
				Token::Value (value) => value,
				_ => panic! ("Called Token::value() on Token::{:?}", self),
			}
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
		"[[[5,[2,8]],4],[5,[[9,9],0]]]",
		"[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
		"[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
		"[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
		"[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
		"[[[[5,4],[7,7]],8],[[8,3],8]]",
		"[[9,3],[[9,9],[6,[4,9]]]]",
		"[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
		"[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (4140, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (3993, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
