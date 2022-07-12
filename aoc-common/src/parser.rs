use super::*;

pub struct Parser <'a, ErrFn> {
	input: & 'a str,
	pos: usize,
	err_fn: ErrFn,
}

impl <'a, ErrFn, ErrFnRet> Parser <'a, ErrFn>
	where ErrFn: Fn (usize) -> ErrFnRet, ErrFnRet: Into <GenError> {

	pub fn new (input: & 'a str, err_fn: ErrFn) -> Parser <'a, ErrFn> {
		Parser {
			input,
			pos: 0,
			err_fn,
		}
	}

	pub fn expect (& mut self, expect: & str) -> GenResult <& mut Self> {
		for expect_char in expect.chars () {
			if self.peek () != Some (expect_char) { Err (self.err ()) ? }
			self.next ();
		}
		Ok (self)
	}

	pub fn int <IntType> (& mut self) -> GenResult <IntType> where IntType: FromStr {
		let len = self.input.chars ().enumerate ()
			.take_while (|& (idx, letter)| letter.is_digit (10) || (idx == 0 && letter == '-'))
			.map (|(_, letter)| letter.len_utf8 ())
			.sum ();
		let val = self.input [0 .. len].parse ().map_err (|_| self.err ()) ?;
		self.input = & self.input [len .. ];
		Ok (val)
	}

	pub fn word (& mut self) -> GenResult <& str> {
		self.skip_whitespace ();
		let input_temp = self.input;
		let start = self.pos;
		while let Some (letter) = self.peek () {
			if letter.is_whitespace () { break }
			self.next ().unwrap ();
		}
		let end = self.pos;
		if start == end { Err (self.err ()) ? }
		self.skip_whitespace ();
		Ok (& input_temp [ .. end - start])
	}

	pub fn skip_whitespace (& mut self) -> & str {
		let input_temp = self.input;
		let start = self.pos;
		while let Some (letter) = self.peek () {
			if ! letter.is_whitespace () { break }
			self.next ().unwrap ();
		}
		let end = self.pos;
		& input_temp [ .. end - start]
	}

	pub fn end (& mut self) -> GenResult <()> {
		if self.peek ().is_some () { Err (self.err ()) ? }
		Ok (())
	}

	pub fn peek (& mut self) -> Option <char> {
		self.input.chars ().next ()
	}

	pub fn next (& mut self) -> Option <char> {
		let letter_opt = self.input.chars ().next ();
		if let Some (letter) = letter_opt {
			self.input = & self.input [letter.len_utf8 () .. ];
			self.pos += 1;
		}
		letter_opt
	}

	pub fn err (& self) -> GenError {
		(self.err_fn) (self.pos).into ()
	}

}
