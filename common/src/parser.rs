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

pub trait ResultParser <Item> {

	/// Map error from [`ParseError`] to `Box <dyn Error>` using the provided function
	///
	#[ allow (clippy::missing_errors_doc) ]
	fn map_parse_err <MapFn, IntoGenErr> (self, map_fn: MapFn) -> GenResult <Item>
		where
			MapFn: Fn (usize) -> IntoGenErr,
			IntoGenErr: Into <GenError>;

	fn map_parse_err_line (self, line_idx: usize, line: & str) -> GenResult <Item>;
	fn map_parse_err_col (self, line: & str) -> GenResult <Item>;

}

impl <Item> ResultParser <Item> for Result <Item, ParseError> {

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

	#[ inline ]
	fn map_parse_err_line (self, line_idx: usize, line: & str) -> GenResult <Item> {
		self.map_parse_err (|col_idx|
			format! ("Invalid input: line {}: col {}: {}", line_idx + 1, col_idx + 1, line))
	}

	#[ inline ]
	fn map_parse_err_col (self, line: & str) -> GenResult <Item> {
		self.map_parse_err (|col_idx|
			format! ("Invalid input: col {}: {}", col_idx + 1, line))
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

	/// Consume and return a decimal integer from the input
	///
	/// This consumes a string of the form [-+]?[0-9]+ from the input and calls [`str::parse`] to
	/// convert it to the specified type.
	///
	/// # Errors
	///
	/// Returns `Err (self.err ())` if `parse` returns an `Err`
	///
	#[ inline ]
	pub fn int <IntType> (& mut self) -> ParseResult <IntType> where IntType: FromStr {
		self.int_real ().parse ().map_err (|_err| self.err ())
	}

	/// Consume and return an unsigned decimal integer from the input
	///
	/// This consumes a string of the form [0-9]+ from the input and calls [`str::parse`] to
	/// convert it to the specified type.
	///
	/// This will actually work with signed integers, although note that it will only match digits
	/// and never a leading minus sign.
	///
	/// # Errors
	///
	/// Returns `Err (self.err ())` if `parse` returns an `Err`
	///
	#[ inline ]
	pub fn uint <IntType> (& mut self) -> ParseResult <IntType> where IntType: FromStr {
		self.uint_real ().parse ().map_err (|_err| self.err ())
	}

	#[ inline ]
	pub fn item <'par, Item> (& 'par mut self) -> ParseResult <Item>
			where Item: FromParser <'inp> {
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

	#[ allow (clippy::string_slice) ]
	fn uint_real (& mut self) -> & str {
		if self.ignore_whitespace { self.skip_whitespace (); }
		let len =
			self.input.chars ()
				.take_while (|& letter| letter.is_ascii_digit ())
				.map (char::len_utf8)
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

	#[ inline ]
	#[ must_use ]
	pub const fn is_empty (& self) -> bool {
		self.input.is_empty ()
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
			WrapFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> {
		let mut parser = Parser::new (input);
		wrap_fn (& mut parser)
	}

	#[ inline ]
	pub fn wrap_auto <Output> (
		input: & 'inp str,
		mut wrap_fn: impl FnMut (& mut Parser <'inp>) -> ParseResult <Output>,
	) -> GenResult <Output> {
		Self::wrap (input, & mut wrap_fn)
			.map_parse_err_col (input)
	}

	#[ inline ]
	pub fn wrap_lines_auto <Output> (
		input: impl Iterator <Item = (usize, & 'inp str)>,
		mut wrap_fn: impl FnMut (& mut Parser <'inp>) -> ParseResult <Output>,
	) -> GenResult <Vec <Output>> {
		input
			.map (|(line_idx, line)| Self::wrap (line, & mut wrap_fn)
				.map_parse_err_line (line_idx, line))
			.collect ()
	}

	#[ inline ]
	pub fn wrap_lines_auto_items <Output: FromParser <'inp>> (
		input: impl Iterator <Item = (usize, & 'inp str)>,
	) -> GenResult <Vec <Output>> {
		Self::wrap_lines_auto (input, |parser| {
			let item = parser.item () ?;
			parser.end () ?;
			Ok (item)
		})
	}

	#[ inline ]
	#[ must_use ]
	pub const fn rest (& self) -> & str {
		self.input
	}

	#[ inline ]
	pub fn delim_fn <'par, Output, ParseFn> (
		& 'par mut self,
		delim: & 'par str,
		parse_fn: ParseFn,
	) -> ParserDelim <'par, 'inp, Output, ParseFn>
			where ParseFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> {
		ParserDelim {
			parser: self,
			delim,
			parse_fn,
			first: true,
		}
	}

	/*
	#[ inline ]
	pub fn delim_items <'par,, Output: FromParser <'inp> + 'out> (
		& 'par mut self,
		delim: & 'par str,
	) -> impl Iterator <Item = ParseResult <Output>> + 'par + 'inp {
		self.delim_fn (delim, Parser::item)
	}

	#[ inline ]
	pub fn delim_uints <'par, 'out: 'par, Output: FromStr + 'out> (
		& 'par mut self,
		delim: & 'par str,
	) -> impl Iterator <Item = ParseResult <Output>> + 'par + 'inp {
		self.delim_fn (delim, |parser| parser.uint ())
	}

	#[ inline ]
	pub fn delim_ints <Output: FromStr + 'static> (
		& 'par mut self,
		delim: & 'par str,
	) -> impl Iterator <Item = ParseResult <Output>> + 'par {
		self.delim_fn (delim, Parser::int)
	}
	*/

}

pub struct ParserDelim <
	'par,
	'inp,
	Output,
	ParseFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> ,
> {
	parser: & 'par mut Parser <'inp>,
	delim: & 'par str,
	parse_fn: ParseFn,
	first: bool,
}

impl <'par, 'inp, Output, ParseFn> Iterator for ParserDelim <'par, 'inp, Output, ParseFn>
		where ParseFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> {

	type Item = ParseResult <Output>;

	#[ inline ]
	fn next (& mut self) -> Option <ParseResult <Output>> {
		if ! self.first {
			if self.parser.expect (self.delim).is_err () { return None }
		} else { self.first = false; }
		Some ((self.parse_fn) (self.parser))
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
pub trait FromParser <'inp>: Sized {

	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self>;

	#[ inline ]
	fn parse_from_str (input: & 'inp str) -> GenResult <Self> {
		Parser::wrap_auto (input, Parser::item)
	}

}

#[ macro_export ]
macro_rules! parse_display_enum {
	(
		$( #[ $($attrs:tt)* ] )*
		$vis:vis enum $enum_name:ident {
			$( $mem_name:ident = $mem_str:literal , )*
		}
	) => {

		$( #[ $($attrs)* ] )*
		$vis enum $enum_name {
			$( $mem_name, )*
		}

		impl ::std::fmt::Display for $enum_name {
			fn fmt (
				& self,
				formatter: & mut ::std::fmt::Formatter,
			) -> ::std::fmt::Result {
				write! (formatter, "{}", match * self {
					$( Self::$mem_name => $mem_str, )*
				}) ?;
				Ok (())
			}
		}

		impl <'inp> ::aoc_common::parser::FromParser <'inp> for $enum_name {
			fn from_parser (
				parser: & mut ::aoc_common::parser::Parser <'inp>,
			) -> ::aoc_common::parser::ParseResult <Self> {
				parser.any ()
					$( .of (|parser| {
						parser.expect ($mem_str) ?;
						Ok (Self::$mem_name)
					}) ) *
					.done ()
			}
		}

	};
}

#[ macro_export ]
macro_rules! input_params {
	(
		$( #[ $($attrs:tt)* ] )*
		pub struct $struct_name:ident {
			$(
				pub $member_name:ident: $member_type:ty =
					($member_prefix:literal, $member_default:literal, $member_range:expr),
			)*
		}
	) => {

		$( #[ $($attrs)* ] )*
		pub struct $struct_name {
			$( pub $member_name: $member_type, )*
		}

		impl $struct_name {
			pub fn parse (input: & mut & [& str]) -> ::aoc_common::GenResult <Self> {
				use ::aoc_common::parser as parser;
				use ::std::ops::Bound as Bound;
				use ::std::ops::RangeBounds as _;
				use ::std::result::Result as Result;
				let default = Self::default ();
				$(
					let $member_name =
						parser::input_param (
							input,
							$member_prefix,
							default.$member_name) ?;
					if ! $member_range.contains (& $member_name) {
						match ($member_range.start_bound (), $member_range.end_bound ()) {
							(Bound::Included (start), Bound::Included (end)) =>
								return Result::Err (format! (
									"{} must be between {} and {}, but was {}",
									& $member_prefix [0 .. $member_prefix.len () - 1],
									start,
									end,
									$member_name,
								).into ()),
							(Bound::Included (start), Bound::Unbounded) =>
								return Result::Err (format! (
									"{} must be at least {}, but was {}",
									& $member_prefix [0 .. $member_prefix.len () - 1],
									start,
									$member_name,
								).into ()),
							(Bound::Excluded (start), Bound::Unbounded) =>
								return Result::Err (format! (
									"{} must be more than {}, but was {}",
									& $member_prefix [0 .. $member_prefix.len () - 1],
									start,
									$member_name,
								).into ()),
							(Bound::Unbounded, Bound::Included (end)) =>
								return Result::Err (format! (
									"{} must be at most {}, but was {}",
									& $member_prefix [0 .. $member_prefix.len () - 1],
									end,
									$member_name,
								).into ()),
							(Bound::Unbounded, Bound::Excluded (end)) =>
								return Result::Err (format! (
									"{} must be less than {}, but was {}",
									& $member_prefix [0 .. $member_prefix.len () - 1],
									end,
									$member_name,
								).into ()),
							_ =>
								return Result::Err (format! (
									"{} is out of acceptable range: {}",
									& $member_prefix [0 .. $member_prefix.len () - 1],
									$member_name,
								).into ()),
						}
					}
				)*
				Ok (Self { $( $member_name, )* })
			}
		}

		impl ::std::default::Default for $struct_name {
			fn default () -> Self {
				Self {
					$( $member_name: $member_default, )*
				}
			}
		}

		impl ::std::fmt::Display for $struct_name {
			fn fmt (
				& self,
				formatter: & mut ::std::fmt::Formatter,
			) -> ::std::fmt::Result {
				let default = Self::default ();
				$(
					if self.$member_name != default.$member_name {
						write! (formatter, "{}{}\n", $member_prefix, self.$member_name) ?;
					}
				)*
				Ok (())
			}
		}

	};
}
