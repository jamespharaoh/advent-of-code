use aoc_inpstr::*;
use aoc_misc::*;
use aoc_nums as nums;

pub type ParseResult <Item> = Result <Item, ParseError>;

#[ derive (Clone) ]
pub struct Parser <'inp> {
	input_line: & 'inp str,
	input_lines: & 'inp [& 'inp str],
	line_idx: usize,
	col_idx: usize,
	byte_idx: usize,
	word_pred: fn (char) -> bool,
	ignore_whitespace: bool,
	confirmed: bool,
}

#[ derive (Debug) ]
pub enum ParseError {
	Simple (usize, usize),
	Wrapped (GenError),
}

pub trait ResultParser <Item> {

	/// Map error from [`ParseError`] to `Box <dyn Error>` using the provided function
	///
	#[ allow (clippy::missing_errors_doc) ]
	fn map_parse_err <MapFn, IntoGenErr> (self, map_fn: MapFn) -> GenResult <Item>
		where
			MapFn: FnOnce (usize, usize) -> IntoGenErr,
			IntoGenErr: Into <GenError>;

	fn map_parse_err_auto (self, parser: & Parser) -> GenResult <Item>;

	fn map_parse_err_line (self, line_idx: usize, line: & str) -> GenResult <Item>;

}

impl <Item> ResultParser <Item> for Result <Item, ParseError> {

	#[ inline ]
	fn map_parse_err <MapFn, IntoGenErr> (self, map_fn: MapFn) -> GenResult <Item>
		where
			MapFn: FnOnce (usize, usize) -> IntoGenErr,
			IntoGenErr: Into <GenError> {
		match self {
			Ok (item) => Ok (item),
			Err (ParseError::Simple (line_idx, col_idx)) =>
				Err (map_fn (line_idx, col_idx).into ()),
			Err (ParseError::Wrapped (err)) => Err (err),
		}
	}

	#[ inline ]
	fn map_parse_err_auto (self, parser: & Parser) -> GenResult <Item> {
		self.map_parse_err (|line_idx, col_idx| {
			let line = parser.input_lines [line_idx];
			format! ("Invalid input: line {}: col {}: {}", line_idx + 1, col_idx + 1, line)
		})
	}

	#[ inline ]
	fn map_parse_err_line (self, line_idx: usize, line: & str) -> GenResult <Item> {
		self.map_parse_err (|_, col_idx|
			format! ("Invalid input: line {}: col {}: {}", line_idx + 1, col_idx + 1, line))
	}

}

impl Display for ParseError {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Simple (line_idx, col_idx) =>
				write! (formatter, "Parser error at line {}, col {}", line_idx + 1, col_idx + 1) ?,
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

impl From <String> for ParseError {

	#[ inline ]
	fn from (other: String) -> Self {
		Self::Wrapped (other.into ())
	}

}

impl <'inp> Parser <'inp> {

	#[ inline ]
	#[ must_use ]
	pub fn new (input_line: & 'inp str) -> Parser <'inp> {
		Parser {
			input_line,
			input_lines: & [],
			line_idx: 0,
			col_idx: 0,
			byte_idx: 0,
			word_pred: |ch| ! ch.is_whitespace (),
			ignore_whitespace: false,
			confirmed: false,
		}
	}

	#[ inline ]
	#[ must_use ]
	pub fn new_lines (input_lines: & 'inp [& 'inp str]) -> Parser <'inp> {
		Parser {
			input_line: input_lines.first ().copied ().unwrap_or (""),
			input_lines,
			line_idx: 0,
			col_idx: 0,
			byte_idx: 0,
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
			if self.peek () != Some (expect_char) { return Err (self.err ()) }
			self.next ().unwrap ();
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
		self.any ().of (|parser| {
			let val_str = parser.int_real ();
			IntType::from_str (val_str).map_err (|_err| parser.err ())
		}).done ()
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
		let (num_chars, num_bytes) =
			self.input_line.chars ()
				.enumerate ()
				.take_while (|& (idx, letter)|
					letter.is_ascii_digit () || (idx == 0 && (letter == '-' || letter == '+')))
				.map (|(_, letter)| letter.len_utf8 ())
				.fold ((0_u32, 0), |(num_chars, num_bytes), ch_bytes|
					(num_chars + 1, num_bytes + ch_bytes));
		let val = & self.input_line [ .. num_bytes];
		for _ in 0 .. num_chars { self.next ().unwrap (); }
		if self.ignore_whitespace { self.skip_whitespace (); }
		val
	}

	#[ allow (clippy::string_slice) ]
	fn uint_real (& mut self) -> & str {
		if self.ignore_whitespace { self.skip_whitespace (); }
		let (num_chars, num_bytes) =
			self.input_line.chars ()
				.take_while (|& letter| letter.is_ascii_digit ())
				.map (char::len_utf8)
				.fold ((0_u32, 0), |(num_chars, num_bytes), ch_bytes|
					(num_chars + 1, num_bytes + ch_bytes));
		let val = & self.input_line [ .. num_bytes];
		for _ in 0 .. num_chars { self.next ().unwrap (); }
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
		let (num_chars, num_bytes) =
			self.input_line.chars ()
				.take_while (|& ch| (self.word_pred) (ch))
				.map (char::len_utf8)
				.fold ((0_u32, 0), |(num_chars, num_bytes), ch_bytes|
					(num_chars + 1, num_bytes + ch_bytes));
		if num_chars == 0 { return Err (self.err ()) }
		let word = & self.input_line [ .. num_bytes];
		for _ in 0 .. num_chars { self.next ().unwrap (); }
		if self.ignore_whitespace { self.skip_whitespace (); }
		Ok (word)
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
	pub fn word_if <'par_1> (
		& 'par_1 mut self,
		pred: impl FnOnce (& 'inp str) -> bool,
	) -> ParseResult <& 'inp str> {
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
		if self.input_line.is_empty () && self.line_idx + 1 < self.input_lines.len () {
			self.line_idx += 1;
			self.input_line = self.input_lines [self.line_idx];
			self.col_idx = 0;
			return Some ('\n');
		}
		let ch = self.input_line.chars ().next () ?;
		self.input_line = & self.input_line [ch.len_utf8 () .. ];
		self.col_idx += 1;
		self.byte_idx += ch.len_utf8 ();
		Some (ch)
	}

	/// Return the next character from the input without consuming it
	///
	#[ inline ]
	pub fn peek (& mut self) -> Option <char> {
		if let Some (ch) = self.input_line.chars ().next () { return Some (ch) }
		if self.line_idx + 1 < self.input_lines.len () { return Some ('\n') }
		None
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
		ParseError::Simple (self.line_idx, self.col_idx)
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
			where WrapFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> {
		let mut parser = Parser::new (input);
		wrap_fn (& mut parser)
	}

	#[ inline ]
	pub fn wrap_auto <Output> (
		input: & 'inp str,
		mut wrap_fn: impl FnMut (& mut Parser <'inp>) -> ParseResult <Output>,
	) -> GenResult <Output> {
		Self::wrap (input, |parser| {
			let item = wrap_fn (parser) ?;
			parser.end () ?;
			Ok (item)
		}).map_parse_err_line (0, input)
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
	pub fn wrap_lines <Output> (
		input_lines: & 'inp [& 'inp str],
		mut wrap_fn: impl FnMut (& mut Parser <'inp>) -> ParseResult <Output>,
	) -> GenResult <Output> {
		let mut parser = Parser::new_lines (input_lines);
		let item = wrap_fn (& mut parser).map_parse_err_auto (& parser) ?;
		parser.end ().map_parse_err_auto (& parser) ?;
		Ok (item)
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
	pub const fn peek_rest (& self) -> & 'inp str {
		self.input_line
	}

	#[ inline ]
	#[ must_use ]
	pub fn take_rest (& mut self) -> InpStr <'inp> {
		let result = self.input_line;
		self.input_line = "";
		InpStr::borrow (result)
	}

	#[ inline ]
	pub fn take_rest_while (
		& mut self,
		char_pred: fn (char) -> bool,
		len: impl RangeBounds <usize>,
	) -> ParseResult <InpStr <'inp>> {
		let result = self.input_line;
		let mut num_bytes = 0_usize;
		let mut num_chars = 0_usize;
		while let Some (ch) = self.peek () {
			if ! (Unbounded, len.end_bound ()).contains (& (num_chars + 1)) { break }
			if ! char_pred (ch) { break }
			self.next ().unwrap ();
			num_bytes += ch.len_utf8 ();
			num_chars += 1;
		}
		if ! (len.start_bound (), Unbounded).contains (& num_chars) {
			return Err (self.err ());
		}
		#[ allow (clippy::string_slice) ]
		Ok (InpStr::borrow (& result [ .. num_bytes]))
	}

	#[ inline ]
	pub fn delim_fn <'par, Output, ParseFn> (
		& 'par mut self,
		delim: & 'par str,
		parse_fn: ParseFn,
	) -> ParserDelim <'par, 'inp, Output, ParseFn>
			where ParseFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> {
		assert! (! delim.is_empty ());
		ParserDelim {
			parser: self,
			delim,
			parse_fn,
			first: true,
		}
	}

	#[ inline ]
	pub fn repeat <'par, Output, ParseFn> (
		& 'par mut self,
		parse_fn: ParseFn,
	) -> ParserRepeat <'par, 'inp, Output, ParseFn>
			where ParseFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> {
		ParserRepeat {
			parser: self,
			parse_fn,
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

pub struct ParserRepeat <
	'par,
	'inp,
	Output,
	ParseFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> ,
> {
	parser: & 'par mut Parser <'inp>,
	parse_fn: ParseFn,
}

impl <'par, 'inp, Output, ParseFn> Iterator for ParserRepeat <'par, 'inp, Output, ParseFn>
		where ParseFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> {

	type Item = Output;

	#[ inline ]
	fn next (& mut self) -> Option <Output> {
		self.parser.any ().of (& mut self.parse_fn).done ().ok ()
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
						parser.input_line = sub_parser.input_line;
						parser.line_idx = sub_parser.line_idx;
						parser.col_idx = sub_parser.col_idx;
						parser.byte_idx = sub_parser.byte_idx;
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

	#[ inline ]
	fn parse_from_lines (input: & 'inp [& 'inp str]) -> GenResult <Self> {
		Parser::wrap_lines (input, Parser::item)
	}

}

#[ macro_export ]
macro_rules! parse_display_enum {
	( $(
		$( #[ $($enum_attrs:tt)* ] )*
		$enum_vis:vis enum $enum_name:ident {
			$(
				$( #[ $($var_attrs:tt)* ] )*
				$var_name:ident = $var_str:literal
			),*
			$(,)?
		}
	)* ) => { $(

		$( #[ $($enum_attrs)* ] )*
		$enum_vis enum $enum_name {
			$(
				$( #[ $($var_attrs)* ] )*
				$var_name,
			)*
		}

		impl $enum_name {
			#[ inline ]
			pub fn as_str (& self) -> & 'static str {
				match * self {
					$( Self::$var_name => $var_str, )*
				}
			}
		}

		impl ::std::fmt::Display for $enum_name {
			fn fmt (
				& self,
				formatter: & mut ::std::fmt::Formatter,
			) -> ::std::fmt::Result {
				write! (formatter, "{}", match * self {
					$( Self::$var_name => $var_str, )*
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
						parser.expect ($var_str) ?;
						Ok (Self::$var_name)
					}) ) *
					.done ()
			}
		}

	)* };
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

		impl <'inp> FromParser <'inp> for $struct_name {
			fn from_parser (parser: & mut Parser <'inp>) -> ::aoc_common::parser::ParseResult <Self> {
				use ::aoc_common::parser as parser;
				use ::std::ops::Bound as Bound;
				use ::std::ops::RangeBounds as _;
				use ::std::result::Result as Result;
				let default = Self::default ();
				$(
					let $member_name = parser.any ().of (|parser| {
						parse! (parser, $member_prefix, val, "\n");
						Ok (val)
					}).done ().unwrap_or (default.$member_name);
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

	};
}

#[ macro_export ]
macro_rules! parse {
	( $parser:expr $(, $item:tt)* $(,)? ) => {
		$( parse! (@item $parser, $item); )*
	};
	( @item $parser:expr, $expect_str:literal ) => {
		$parser.expect ($expect_str) ?;
	};
	( @item $parser:expr, $item_name:ident ) => {
		let $item_name = $parser.item () ?;
	};
	( @item $parser:expr, ($item_name:ident: $item_type:ty) ) => {
		let $item_name: $item_type = $parser.item () ?;
	};
	( @item $parser:expr, (@uint $item_name:ident: $item_type:ty) ) => {
		let $item_name: $item_type = $parser.uint () ?;
	};
	( @item $parser:expr, (@int $item_name:ident: $item_type:ty) ) => {
		let $item_name: $item_type = $parser.int () ?;
	};
	( @item $parser:expr, ($item_name:ident = $item_parse:ident) ) => {
		let $item_name = $item_parse ($parser) ?;
	};
	( @item $parser:expr, ($item_name:ident = $item_range:expr) ) => {
		let $item_name = $parser.any ().of (|parser| {
			let val = parser.item () ?;
			if ! $item_range.contains (& val) { return Err (parser.err ()) }
			Ok (val)
		}).done () ?;
	};
	( @item $parser:expr, (@rest $item_name:ident) ) => {
		let $item_name = $parser.take_rest ();
	};
	( @item $parser:expr, (@rest $item_name:ident = $item_ch_pred:expr) ) => {
		let $item_name = $parser.take_rest_while ($item_ch_pred, .. ) ?;
	};
	( @item $parser:expr, (@rest $item_name:ident = $item_ch_pred:expr, $item_len:expr) ) => {
		let $item_name = $parser.take_rest_while ($item_ch_pred, $item_len) ?;
	};
	( @item $parser:expr, (@collect $item_name:ident) ) => {
		let $item_name = $parser
			.repeat (Parser::item)
			.collect ();
	};
	( @item $parser:expr, (@collect $item_name:ident: $item_type:ty) ) => {
		let $item_name: $item_type = $parser
			.repeat (Parser::item)
			.collect ();
	};
	( @item $parser:expr, (@collect $item_name:ident = $item_parse:expr) ) => {
		let $item_name = $parser
			.repeat ($item_parse)
			.collect () ?;
	};
	( @item $parser:expr, (@delim_items $delim:literal $item_name:ident) ) => {
		let $item_name = $parser
			.delim_fn ($delim, Parser::item)
			.try_collect () ?;
	};
	( @item $parser:expr, (@line_items $item_name:ident) ) => {
		let $item_name = $parser
			.delim_fn ("\n", Parser::item)
			.try_collect () ?;
	};
	( @item $parser:expr, (@line_items $item_name:ident = $item_parse:ident) ) => {
		let $item_name = $parser
			.delim_fn ("\n", $item_parse)
			.try_collect () ?;
	};
	( @item $parser:expr, (@lines $item_name:ident) ) => {
		let $item_name = $parser
			.delim_fn ("\n", |parser| Ok (parser.take_rest ()))
			.try_collect () ?;
	};
	( @item $parser:expr, (@lines $item_name:ident = $item_ch_pred:expr) ) => {
		let $item_name = $parser
			.delim_fn ("\n", |parser| parser.take_rest_while ($item_ch_pred, .. ))
			.try_collect () ?;
	};
	( @item $parser:expr, (@end) ) => {
		$parser.end () ?;
	};
	( @item $parser:expr, (@confirm) ) => {
		$parser.confirm ();
	};
	( @item $parser:expr, (@skip) ) => {
		$parser.skip_whitespace ();
	};
}

macro_rules! from_parser_impl {
	( $name:ident, $method:ident ) => {
		impl <'inp> FromParser <'inp> for $name {
			#[ inline ]
			fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <$name> {
				parser.$method ()
			}
		}
	};
}

from_parser_impl! (i8, int);
from_parser_impl! (i16, int);
from_parser_impl! (i32, int);
from_parser_impl! (i64, int);
from_parser_impl! (i128, int);

from_parser_impl! (u8, uint);
from_parser_impl! (u16, uint);
from_parser_impl! (u32, uint);
from_parser_impl! (u64, uint);
from_parser_impl! (u128, uint);

from_parser_impl! (char, expect_next);

impl <'inp> FromParser <'inp> for bool {

	#[ inline ]
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parser.any ()
			.of (|parser| { parser.expect ("true") ?; Ok (true) })
			.of (|parser| { parser.expect ("false") ?; Ok (false) })
			.done ()
	}

}

impl <'inp> FromParser <'inp> for InpStr <'inp> {

	#[ inline ]
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		Ok (parser.take_rest ())
	}

}
