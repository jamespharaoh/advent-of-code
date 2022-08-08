use super::*;

pub type ParseResult <Item> = Result <Item, ParseError>;

#[ derive (Clone) ]
pub struct Parser <'inp> {
	input: & 'inp str,
	pos: usize,
	word_pred: fn (char) -> bool,
	ignore_whitespace: bool,
	confirmed: bool,
}

#[ derive (Debug) ]
pub enum ParseError {
	Simple (usize),
	Wrapped (GenError),
}

pub trait ResultExt <Item> {

	/// Map error from [`ParseError`] to `Box <dyn Error>` using the provided function
	///
	#[ allow (clippy::missing_errors_doc) ]
	fn map_parse_err <MapFn, IntoGenErr> (self, map_fn: MapFn) -> GenResult <Item>
		where
			MapFn: Fn (usize) -> IntoGenErr,
			IntoGenErr: Into <GenError>;

}

impl <Item> ResultExt <Item> for Result <Item, ParseError> {

	#[ inline ]
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

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Simple (char_idx) =>
				write! (formatter, "Parser error at col {}", char_idx + 1) ?,
			Self::Wrapped (ref inner) =>
				Display::fmt (inner, formatter) ?,
		}
		Ok (())
	}

}

impl Error for ParseError {
}

impl From <GenError> for ParseError {

	#[ inline ]
    fn from (other: GenError) -> Self {
        Self::Wrapped (other)
    }

}

impl From <nums::Overflow> for ParseError {

	#[ inline ]
	fn from (other: nums::Overflow) -> Self {
		Self::Wrapped (Box::new (other))
	}

}

impl From <& str> for ParseError {

	#[ inline ]
	fn from (other: & str) -> Self {
		Self::Wrapped (other.into ())
	}

}

impl <'inp> Parser <'inp> {

	#[ inline ]
	#[ must_use ]
	pub fn new (input: & 'inp str) -> Parser <'inp> {
		Parser {
			input,
			pos: 0,
			word_pred: |ch| ! ch.is_whitespace (),
			ignore_whitespace: false,
			confirmed: false,
		}
	}

	#[ inline ]
	pub fn confirm (& mut self) -> & mut Self {
		self.confirmed = true;
		self
	}

	#[ inline ]
	pub fn set_ignore_whitespace (& mut self, ignore_whitespace: bool) -> & mut Self {
		self.ignore_whitespace = ignore_whitespace;
		self
	}

	#[ inline ]
	pub fn set_word_pred (& mut self, word_pred: fn (char) -> bool) -> & mut Self {
		self.word_pred = word_pred;
		self
	}

	/// Consume a specific string from the input
	///
	/// # Errors
	///
	/// Returns `Err (self.err ())` if the input does not match the specified value
	///
	#[ inline ]
	pub fn expect (& mut self, expect: & str) -> ParseResult <& mut Self> {
		if self.ignore_whitespace { self.skip_whitespace (); }
		for expect_char in expect.chars () {
			if self.peek () != Some (expect_char) { Err (self.err ()) ? }
			self.next ();
		}
		if self.ignore_whitespace { self.skip_whitespace (); }
		Ok (self)
	}

	/// Consume a specific word from the input
	///
	/// # Errors
	///
	/// Returns `Err (self.err ())` if the input does not match the specified value
	///
	#[ inline ]
	pub fn expect_word (& mut self, expect: & str) -> ParseResult <& mut Self> {
		if self.word () ? != expect { Err (self.err ()) ? }
		Ok (self)
	}

	/// Consume and return a decimal number from the input
	///
	/// In fact, this will work for any type which implements [`FromStr`], but it is intended for
	/// use with numbers.
	///
	/// # Errors
	///
	/// Returns `Err (self.err ())` if parse returns an `Err`
	///
	#[ inline ]
	pub fn int <IntType> (& mut self) -> ParseResult <IntType> where IntType: FromStr {
		self.int_real ().parse ().map_err (|_err| self.err ())
	}

	#[ inline ]
	pub fn item <Item> (& mut self) -> ParseResult <Item> where Item: FromParser {
		Item::from_parser (self)
	}

	#[ allow (clippy::string_slice) ]
	fn int_real (& mut self) -> & str {
		if self.ignore_whitespace { self.skip_whitespace (); }
		let len =
			self.input.chars ()
				.enumerate ()
				.take_while (|& (idx, letter)|
					letter.is_ascii_digit () || (idx == 0 && (letter == '-' || letter == '+')))
				.map (|(_, letter)| letter.len_utf8 ())
				.sum ();
		let val = & self.input [0 .. len];
		self.input = & self.input [len .. ];
		if self.ignore_whitespace { self.skip_whitespace (); }
		val
	}

	/// Consume and return a single word from the input
	///
	/// # Errors
	///
	/// Returns `Err (self.err ())` if there is no word remaining
	///
	#[ allow (clippy::missing_inline_in_public_items) ]
	#[ allow (clippy::string_slice) ]
	pub fn word (& mut self) -> ParseResult <& 'inp str> {
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

	/// Consume and return a single word from the input, transforming it with [`TryInto::into`]
	///
	/// # Errors
	///
	/// Returns `Err (self.err ())` if there is no word remaining.
	///
	/// Returns `Err (ParseError::Wrapped (err))` if the conversion fails.
	///
	#[ inline ]
	pub fn word_into <'par_1, Output> (& 'par_1 mut self) -> ParseResult <Output>
			where Output: TryFrom <& 'par_1 str, Error = GenError> {
		Ok (self.word () ?.try_into () ?)
	}

	/// Consume and return a single word from the input, validating it with the provided function
	///
	/// # Errors
	///
	/// Returns `Err (self.err ())` if there is no word remaining, or if the provided predicate
	/// function returns false
	///
	#[ inline ]
	pub fn word_if <'par_1, PredFn> (
		& 'par_1 mut self,
		pred: PredFn,
	) -> ParseResult <& 'inp str>
			where PredFn: Fn (& 'inp str) -> bool {
		let word = self.word () ?;
		if ! pred (word) { Err (self.err ()) ?; }
		Ok (word)
	}

	/// Return a word from the input without consuming it
	///
	#[ inline ]
	pub fn peek_word (& mut self) -> Option <& 'inp str> {
		self.clone ().word ().ok ()
	}

	/// Consume any whitespace from the start of the remaining input
	///
	#[ inline ]
	pub fn skip_whitespace (& mut self) -> & mut Self {
		while let Some (letter) = self.peek () {
			if ! letter.is_whitespace () { break }
			self.next ().unwrap ();
		}
		self
	}

	/// Assert that there is no more input to consume
	///
	/// # Errors
	///
	/// Returns `Err (self.err ())` if there is more input.
	///
	#[ inline ]
	pub fn end (& mut self) -> ParseResult <()> {
		if self.peek ().is_some () { Err (self.err ()) ? }
		Ok (())
	}

	/// Consume and return the next character from the input
	///
	#[ allow (clippy::should_implement_trait) ]
	#[ allow (clippy::string_slice) ]
	#[ inline ]
	pub fn next (& mut self) -> Option <char> {
		let letter_opt = self.input.chars ().next ();
		if let Some (letter) = letter_opt {
			self.input = & self.input [letter.len_utf8 () .. ];
			self.pos += letter.len_utf8 ();
		}
		letter_opt
	}

	/// Return the next character from the input without consuming it
	///
	#[ inline ]
	pub fn peek (& mut self) -> Option <char> {
		self.input.chars ().next ()
	}

	/// Consume and return the next character from the input
	///
	/// # Errors
	///
	/// Will return `Err (self.err ())` if there is no input remaining.
	///
	#[ inline ]
	pub fn expect_next (& mut self) -> ParseResult <char> {
		self.next ().ok_or_else (|| self.err ())
	}

	/// Return a `ParseError` with the current position
	///
	#[ inline ]
	#[ must_use ]
	pub const fn err (& self) -> ParseError {
	    ParseError::Simple (self.pos)
	}

	#[ inline ]
	pub fn any <Item> (& mut self) -> ParserAny <'_, 'inp, Item> {
		ParserAny::Parser (self)
	}

	#[ inline ]
	pub fn wrap <Output, WrapFn> (
		input: & 'inp str,
		mut wrap_fn: WrapFn,
	) -> ParseResult <Output>
		where
			WrapFn: FnMut (& mut Parser) -> ParseResult <Output> {
		let mut parser = Parser::new (input);
		wrap_fn (& mut parser)
	}

	#[ inline ]
	#[ must_use ]
	pub const fn rest (& self) -> & str {
		self.input
	}

}

pub enum ParserAny <'par, 'inp, Item> {
	Parser (& 'par mut Parser <'inp>),
	Item (Item),
	ConfirmedError (ParseError),
}

impl <'par, 'inp, Item> ParserAny <'par, 'inp, Item> {

	#[ inline ]
	#[ must_use ]
	pub fn of <OfFn> (self, mut of_fn: OfFn) -> Self
			where OfFn: FnMut (& mut Parser <'inp>) -> ParseResult <Item> {
		match self {
			ParserAny::Parser (parser) => {
				let mut sub_parser = Parser { confirmed: false, .. * parser };
				match of_fn (& mut sub_parser) {
					Ok (item) => {
						parser.input = sub_parser.input;
						parser.pos = sub_parser.pos;
						ParserAny::Item (item)
					},
					Err (err) =>
						if sub_parser.confirmed { ParserAny::ConfirmedError (err) }
						else { ParserAny::Parser (parser) },
				}
			},
			ParserAny::Item (item) => ParserAny::Item (item),
			ParserAny::ConfirmedError (err) => ParserAny::ConfirmedError (err),
		}
	}

	#[ inline ]
	pub fn done (self) -> ParseResult <Item> {
		match self {
			ParserAny::Parser (parser) => Err (parser.err ()),
			ParserAny::Item (item) => Ok (item),
			ParserAny::ConfirmedError (err) => Err (err),
		}
	}

}

/// Utility method to parse a parameter from the start of an input with default value
///
#[ inline ]
pub fn input_param <Val: FromStr, Def: Into <Val>> (
	input: & mut & [& str],
	prefix: & str,
	default: Def,
) -> GenResult <Val>
		where Val::Err: Error + 'static {
	Ok (
		if let Some (line) = input.first () {
			if let Some (val) = line.strip_prefix (prefix) {
				* input = & (* input) [1 .. ];
				val.parse () ?
			} else { default.into () }
		} else { default.into () }
	)
}

/// Utility method to parse an optional parameter from the start of an input
///
#[ inline ]
pub fn input_param_opt <Val: FromStr> (
	input: & mut & [& str],
	prefix: & str,
) -> GenResult <Option <Val>>
		where Val::Err: Error + 'static {
	Ok (
		if let Some (val) = input [0].strip_prefix (prefix) {
			* input = & (* input) [1 .. ];
			Some (val.parse () ?)
		} else { None }
	)
}

/// Utility method to prepend parameters to an example
///
#[ inline ]
#[ must_use ]
pub fn with_params <const LEN: usize> (
	params: [& 'static str; LEN],
	example: & [& 'static str],
) -> Vec <& 'static str> {
	params.into_iter ()
		.chain (example.iter ().copied ())
		.collect ()
}

/// Trait implemented by types which can be produced by [`Parser::item`]
///
pub trait FromParser: Sized {
	fn from_parser (parser: & mut Parser) -> ParseResult <Self>;
}
