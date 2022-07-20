use super::*;

pub type ParseResult <Item> = Result <Item, ParseError>;

#[ derive (Clone) ]
pub struct Parser <'inp> {
	input: & 'inp str,
	pos: usize,
	word_pred: fn (char) -> bool,
	ignore_whitespace: bool,
}

#[ derive (Debug) ]
pub enum ParseError {
	Simple (usize),
	Wrapped (GenError),
}

pub trait ResultExt <Item> {
	fn map_parse_err <MapFn, IntoGenErr> (self, map_fn: MapFn) -> GenResult <Item>
		where
			MapFn: Fn (usize) -> IntoGenErr,
			IntoGenErr: Into <GenError>;
}

impl <Item> ResultExt <Item> for Result <Item, ParseError> {
	fn map_parse_err <MapFn, IntoGenErr> (self, map_fn: MapFn) -> GenResult <Item>
		where
			MapFn: Fn (usize) -> IntoGenErr,
			IntoGenErr: Into <GenError> {
		match self {
			Ok (item) => Ok (item),
			Err (ParseError::Simple (char_idx)) => Err (map_fn (char_idx).into ()),
			Err (ParseError::Wrapped (err)) => Err (err),
		}
	}
}

impl Display for ParseError {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match self {
			ParseError::Simple (char_idx) =>
				write! (formatter, "Parser error at col {}", char_idx + 1) ?,
			ParseError::Wrapped (inner) =>
				Display::fmt (inner, formatter) ?,
		}
		Ok (())
	}
}

impl Error for ParseError {
}

impl From <GenError> for ParseError {
    fn from (other: GenError) -> ParseError {
        ParseError::Wrapped (other)
    }
}

impl From <& str> for ParseError {
	fn from (other: & str) -> ParseError {
		ParseError::Wrapped (other.into ())
	}
}

impl <'inp> Parser <'inp> {

	pub fn new (input: & 'inp str) -> Parser <'inp> {
		Parser {
			input,
			pos: 0,
			word_pred: |ch| ! ch.is_whitespace (),
			ignore_whitespace: false,
		}
	}

	pub fn set_ignore_whitespace (& mut self, ignore_whitespace: bool) -> & mut Self {
		self.ignore_whitespace = ignore_whitespace;
		self
	}

	pub fn set_word_pred (& mut self, word_pred: fn (char) -> bool) -> & mut Self {
		self.word_pred = word_pred;
		self
	}

	pub fn expect (& mut self, expect: & str) -> ParseResult <& mut Self> {
		for expect_char in expect.chars () {
			if self.peek () != Some (expect_char) { Err (self.err ()) ? }
			self.next ();
		}
		Ok (self)
	}

	pub fn expect_word (& mut self, expect: & str) -> ParseResult <& mut Self> {
		if self.word () ? != expect { Err (self.err ()) ? }
		Ok (self)
	}

	pub fn int <IntType> (& mut self) -> ParseResult <IntType> where IntType: FromStr {
		if self.ignore_whitespace { self.skip_whitespace (); }
		let len = self.input.chars ().enumerate ()
			.take_while (|& (idx, letter)|
				letter.is_ascii_digit () || (idx == 0 && letter == '-'))
			.map (|(_, letter)| letter.len_utf8 ())
			.sum ();
		let val = self.input [0 .. len].parse ().map_err (|_| self.err ()) ?;
		self.input = & self.input [len .. ];
		if self.ignore_whitespace { self.skip_whitespace (); }
		Ok (val)
	}

	pub fn word <'b> (& 'b mut self) -> ParseResult <& 'inp str> {
		if self.ignore_whitespace { self.skip_whitespace (); }
		let input_temp = self.input;
		let start = self.pos;
		while let Some (letter) = self.peek () {
			if ! (self.word_pred) (letter) { break }
			self.next ().unwrap ();
		}
		let end = self.pos;
		if start == end { Err (self.err ()) ? }
		if self.ignore_whitespace { self.skip_whitespace (); }
		Ok (& input_temp [ .. end - start])
	}

	pub fn word_into <'b, Output> (& 'b mut self) -> ParseResult <Output>
			where Output: TryFrom <& 'b str, Error = GenError> {
		Ok (self.word () ?.try_into () ?)
	}

	pub fn word_if <'b, PredFn> (& 'b mut self, pred: PredFn) -> ParseResult <& 'inp str>
			where PredFn: Fn (& 'inp str) -> bool {
		let word = self.word () ?;
		if ! pred (word) { Err (self.err ()) ?; }
		Ok (word)
	}

	pub fn peek_word (& mut self) -> Option <& 'inp str> {
		self.clone ().word ().ok ()
	}

	pub fn skip_whitespace (& mut self) -> & mut Self {
		while let Some (letter) = self.peek () {
			if ! letter.is_whitespace () { break }
			self.next ().unwrap ();
		}
		self
	}

	pub fn end (& mut self) -> ParseResult <()> {
		if self.peek ().is_some () { Err (self.err ()) ? }
		Ok (())
	}

	#[ allow (clippy::should_implement_trait) ]
	pub fn next (& mut self) -> Option <char> {
		let letter_opt = self.input.chars ().next ();
		if let Some (letter) = letter_opt {
			self.input = & self.input [letter.len_utf8 () .. ];
			self.pos += 1;
		}
		letter_opt
	}

	pub fn peek (& mut self) -> Option <char> {
		self.input.chars ().next ()
	}

	pub fn expect_next (& mut self) -> ParseResult <char> {
		self.next ().ok_or_else (|| self.err ())
	}

	pub fn err (& self) -> ParseError {
	    ParseError::Simple (self.pos)
	}

	pub fn any <Item> (& mut self) -> ParserAny <'_, 'inp, Item> {
		ParserAny::Parser (self)
	}

	pub fn wrap <Output, WrapFn> (input: & str, mut wrap_fn: WrapFn) -> ParseResult <Output>
			where WrapFn: FnMut (& mut Parser) -> ParseResult <Output> {
		let mut parser = Parser::new (input);
		wrap_fn (& mut parser)
	}

}

pub enum ParserAny <'par, 'inp, Item> {
	Parser (& 'par mut Parser <'inp>),
	Item (Item),
}

impl <'par, 'inp, Item> ParserAny <'par, 'inp, Item> {

	pub fn of <OfFn> (self, mut of_fn: OfFn) -> Self
			where OfFn: FnMut (& mut Parser <'inp>) -> ParseResult <Item> {
		match self {
			ParserAny::Parser (parser) => {
				let mut sub_parser = parser.clone ();
				match of_fn (& mut sub_parser) {
					Ok (item) => {
						parser.input = sub_parser.input;
						parser.pos = sub_parser.pos;
						ParserAny::Item (item)
					},
					Err (_) => ParserAny::Parser (parser),
				}
			},
			ParserAny::Item (item) => ParserAny::Item (item),
		}
	}

	pub fn done (self) -> ParseResult <Item> {
		match self {
			ParserAny::Parser (parser) => Err (parser.err ()),
			ParserAny::Item (item) => Ok (item),
		}
	}

}
