use aoc_inpstr::*;
use aoc_misc::prelude::*;
use aoc_nums as nums;

use nums::IntConv;

mod delim;
mod display;
mod enums;
mod from_parser;
mod parse;
mod structs;

pub use delim::*;
pub use display::IntoIteratorDisplayDelim;
pub use from_parser::FromParser;

pub type ParseResult <Item> = Result <Item, ParseError>;

#[ derive (Clone, Copy) ]
pub struct Parser <'inp> {
	input_line: & 'inp str,
	input_lines: & 'inp [& 'inp str],
	line_idx: u32,
	col_idx: u32,
	byte_idx: u32,
	word_pred: fn (char) -> bool,
	ignore_whitespace: bool,
	confirmed: bool,
}

#[ derive (Debug) ]
pub enum ParseError {
	Simple (u32, u32),
	Wrapped (GenError),
}

pub trait ResultParser <Item> {

	/// Map error from [`ParseError`] to `Box <dyn Error>` using the provided function
	///
	#[ allow (clippy::missing_errors_doc) ]
	fn map_parse_err <MapFn, IntoGenErr> (self, map_fn: MapFn) -> GenResult <Item>
		where
			MapFn: FnOnce (u32, u32) -> IntoGenErr,
			IntoGenErr: Into <GenError>;

	fn map_parse_err_auto (self, parser: & Parser) -> GenResult <Item>;

	fn map_parse_err_line (self, line_idx: usize, line: & str) -> GenResult <Item>;

}

impl <Item> ResultParser <Item> for Result <Item, ParseError> {

	#[ inline ]
	fn map_parse_err <MapFn, IntoGenErr> (self, map_fn: MapFn) -> GenResult <Item>
		where
			MapFn: FnOnce (u32, u32) -> IntoGenErr,
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
			let line = parser.input_lines [line_idx.pan_usize ()];
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
	#[ allow (clippy::string_slice) ]
	#[ inline ]
	pub fn expect (& mut self, expect: & str) -> ParseResult <& mut Self> {
		if self.ignore_whitespace { self.skip_whitespace ( .. ).unwrap (); }
		let saved = * self;
		let mut input_iter = self.input_line.bytes ();
		let mut num_chars = 0_u32;
		let mut num_bytes = 0_u32;
		for expect_byte in expect.bytes () {
			match input_iter.next () {
				Some (input_byte) if input_byte == expect_byte => {
					if expect_byte & 0xc0 != 0x80 { num_chars += 1; }
					num_bytes += 1;
				},
				None if expect_byte == b'\n'
						&& self.line_idx.pan_usize () + 1 < self.input_lines.len () => {
					self.line_idx += 1;
					self.col_idx = 0;
					self.byte_idx = 0;
					self.input_line = self.input_lines [self.line_idx.pan_usize ()];
					input_iter = self.input_line.bytes ();
					num_chars = 0;
					num_bytes = 0;
				},
				_ => {
					* self = saved;
					return Err (self.err ());
				},
			}
		}
		self.col_idx += num_chars;
		self.byte_idx += num_bytes;
		self.input_line = & self.input_line [num_bytes.pan_usize () .. ];
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
		let saved = * self;
		self.int_str ().parse ().map_err (|_err| {
			* self = saved;
			self.err ()
		})
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
	pub fn uint <IntType> (
		& mut self,
	) -> ParseResult <IntType> where IntType: FromStr {
		let saved = * self;
		self.uint_str ().parse ().map_err (|_err| { * self = saved; self.err () })
	}

	#[ inline ]
	pub fn item <Item> (& mut self) -> ParseResult <Item>
			where Item: FromParser <'inp> {
		if self.ignore_whitespace { self.skip_whitespace ( .. ).unwrap (); }
		Item::from_parser (self)
	}

	#[ inline ]
	pub fn item_range <Item> (& mut self, range: impl RangeBounds <Item>) -> ParseResult <Item>
			where Item: FromParser <'inp> + PartialOrd {
		if self.ignore_whitespace { self.skip_whitespace ( .. ).unwrap (); }
		let saved = * self;
		let val = Item::from_parser (self) ?;
		if ! range.contains (& val) {
			* self = saved;
			return Err (self.err ());
		}
		Ok (val)
	}

	#[ allow (clippy::string_slice) ]
	fn int_str (& mut self) -> & str {
		if self.ignore_whitespace { self.skip_whitespace ( .. ).unwrap (); }
		let num_digits =
			self.input_line.bytes ().enumerate ()
				.take_while (|& (idx, ch)|
					ch.is_ascii_digit () || (idx == 0 && (ch == b'-' || ch == b'+')))
				.fold (0, |sum, _| sum + 1);
		let val = & self.input_line [ .. num_digits.pan_usize ()];
		self.input_line = & self.input_line [num_digits.pan_usize () .. ];
		self.col_idx += num_digits;
		self.byte_idx += num_digits;
		val
	}

	#[ allow (clippy::string_slice) ]
	fn uint_str (& mut self) -> & str {
		if self.ignore_whitespace { self.skip_whitespace ( .. ).unwrap (); }
		let num_digits =
			self.input_line.bytes ()
				.take_while (|& ch| ch.is_ascii_digit ())
				.fold (0, |sum, _| sum + 1);
		let val = & self.input_line [ .. num_digits.pan_usize ()];
		self.input_line = & self.input_line [num_digits.pan_usize () .. ];
		self.col_idx += num_digits;
		self.byte_idx += num_digits;
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
		if self.ignore_whitespace { self.skip_whitespace ( .. ).unwrap (); }
		let (num_chars, num_bytes) =
			self.input_line.chars ()
				.take_while (|& ch| (self.word_pred) (ch))
				.map (char::len_utf8)
				.fold ((0_u32, 0_usize), |(num_chars, num_bytes), ch_bytes|
					(num_chars + 1, num_bytes + ch_bytes));
		if num_chars == 0 { return Err (self.err ()) }
		let word = & self.input_line [ .. num_bytes.pan_usize ()];
		self.input_line = & self.input_line [num_bytes.pan_usize () .. ];
		self.col_idx += num_chars;
		self.byte_idx += num_bytes.pan_u32 ();
		Ok (word)
	}

	/// Consume and return a single word from the input, transforming it with [`TryInto`]
	///
	/// # Errors
	///
	/// Returns `Err (self.err ())` if there is no word remaining.
	///
	/// Returns `Err (ParseError::Wrapped (err))` if the conversion fails.
	///
	#[ inline ]
	pub fn word_into <'par, Output> (& 'par mut self) -> ParseResult <Output>
			where Output: TryFrom <& 'par str, Error = GenError> {
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
	#[ allow (clippy::missing_inline_in_public_items) ]
	#[ allow (clippy::string_slice) ]
	pub fn skip_whitespace (& mut self, range: impl RangeBounds <u32>) -> ParseResult <& mut Self> {
		let saved = * self;
		let num_spaces =
			self.input_line.bytes ()
				.take_while (|& ch| ch.is_ascii_whitespace ())
				.fold (0_u32, |sum, _| sum + 1);
		if ! range.contains (& num_spaces) { * self = saved; return Err (self.err ()) }
		self.input_line = & self.input_line [num_spaces.pan_usize () .. ];
		self.col_idx += num_spaces;
		self.byte_idx += num_spaces;
		Ok (self)
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
		if self.input_line.is_empty () && self.line_idx + 1 < self.input_lines.len ().pan_u32 () {
			self.line_idx += 1;
			self.input_line = self.input_lines [self.line_idx.pan_usize ()];
			self.col_idx = 0;
			return Some ('\n');
		}
		let ch = self.input_line.chars ().next () ?;
		self.input_line = & self.input_line [ch.len_utf8 () .. ];
		self.col_idx += 1;
		self.byte_idx += ch.len_utf8 ().pan_u32 ();
		Some (ch)
	}

	/// Return the next character from the input without consuming it
	///
	#[ inline ]
	pub fn peek (& mut self) -> Option <char> {
		self.input_line.chars ().next ().or_else (||
			(self.line_idx + 1 < self.input_lines.len ().pan_u32 ()).then_some ('\n'))
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
	pub fn wrap_auto <Output, WrapFn> (
		input: & 'inp str,
		mut wrap_fn: WrapFn,
	) -> GenResult <Output>
			where WrapFn: for <'par1> FnMut (& 'par1 mut Parser <'inp>) -> ParseResult <Output> {
		Self::wrap (input, |parser: & mut Parser <'inp>| {
			let item = wrap_fn (parser) ?;
			parser.end () ?;
			Ok (item)
		}).map_parse_err_line (0, input)
	}

	#[ inline ]
	pub fn wrap_lines_auto <Output, WrapFn> (
		input: impl Iterator <Item = (usize, & 'inp str)>,
		mut wrap_fn: WrapFn,
	) -> GenResult <Vec <Output>>
			where WrapFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> {
		input
			.map (|(line_idx, line)| Self::wrap (line, & mut wrap_fn)
				.map_parse_err_line (line_idx, line))
			.collect ()
	}

	#[ inline ]
	pub fn wrap_lines <Output, WrapFn> (
		input_lines: & 'inp [& 'inp str],
		mut wrap_fn: WrapFn,
	) -> GenResult <Output>
			where WrapFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> {
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

	#[ allow (clippy::string_slice) ]
	#[ inline ]
	pub fn take_rest_while (
		& mut self,
		char_pred: fn (char) -> bool,
		len: impl RangeBounds <u32>,
	) -> ParseResult <InpStr <'inp>> {
		let mut num_bytes = 0_u32;
		let mut num_chars = 0_u32;
		for ch in self.input_line.chars () {
			if ! (Unbounded, len.end_bound ()).contains (& (num_chars + 1)) { break }
			if ! char_pred (ch) { break }
			num_chars += 1;
			num_bytes += ch.len_utf8 ().qck_u32 ();
		}
		if ! (len.start_bound (), Unbounded).contains (& num_chars) {
			return Err (self.err ());
		}
		let result = InpStr::borrow (& self.input_line [ .. num_bytes.qck_usize ()]);
		self.input_line = & self.input_line [num_bytes.qck_usize () .. ];
		self.col_idx += num_chars;
		self.byte_idx += num_bytes;
		Ok (result)
	}

	#[ allow (clippy::missing_inline_in_public_items) ]
	#[ allow (clippy::string_slice) ]
	pub fn take_exactly (& mut self, num_chars: u32) -> ParseResult <InpStr <'inp>> {
		let (num_bytes, num_chars_found) =
			self.input_line.chars ()
				.take (num_chars.pan_usize ())
				.fold ((0_u32, 0_u32), |(num_bytes, num_chars), ch|
					(num_bytes + ch.len_utf8 ().qck_u32 (), num_chars + 1));
		if num_chars_found < num_chars { return Err (self.err ()) }
		let result = InpStr::borrow (& self.input_line [ .. num_bytes.qck_usize ()]);
		self.input_line = & self.input_line [num_bytes.qck_usize () .. ];
		self.col_idx += num_chars;
		self.byte_idx += num_bytes.pan_u32 ();
		Ok (result)
	}
		

	#[ inline ]
	pub fn opt_fn <ParseFn, Output> (& mut self, mut parse_fn: ParseFn) -> Output
		where
			Output: Default,
			ParseFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> {
		let saved = * self;
		if let Ok (val) = parse_fn (self) { return val }
		* self = saved;
		Output::default ()
	}

	#[ inline ]
	pub fn nest <ParseFn, Output> (& mut self, mut parse_fn: ParseFn) -> ParseResult <Output>
			where ParseFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> {
		let saved = * self;
		let result = parse_fn (self);
		self.ignore_whitespace = saved.ignore_whitespace;
		self.word_pred = saved.word_pred;
		result
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

#[ macro_export ]
macro_rules! input_params {
	(
		$( #[ $($attrs:tt)* ] )*
		pub struct $struct_name:ident {
			$(
				pub $member_name:ident: $member_type:ty =
					($member_prefix:literal, $member_default:expr, $member_range:expr),
			)*
		}
	) => {

		$( #[ $($attrs)* ] )*
		pub struct $struct_name {
			$( pub $member_name: $member_type, )*
		}

		impl $struct_name {
			#[ inline ]
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
					::aoc_common::parser::check_range::<$member_type, _> (
						& $member_prefix [ .. $member_prefix.len () - 1],
						$member_name,
						$member_range) ?;
				)*
				Ok (Self { $( $member_name, )* })
			}
		}

		impl ::std::default::Default for $struct_name {
			#[ inline ]
			fn default () -> Self {
				Self {
					$( $member_name: $member_default, )*
				}
			}
		}

		impl ::std::fmt::Display for $struct_name {
			#[ inline ]
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
			#[ inline ]
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
					::aoc_common::parser::check_range::<$member_type, _> (
						& $member_prefix [ .. $member_prefix.len () - 1],
						$member_name,
						$member_range) ?;
				)*
				Ok (Self { $( $member_name, )* })
			}
		}

	};
}

#[ inline ]
pub fn check_range <Val: Debug + Display + Ord, Rng: Debug + RangeBounds <Val>> (
	name: & str,
	val: Val,
	range: Rng,
) -> GenResult <()> {
	if range.contains (& val) { return Ok (()) }
	Err (check_range_error (name, val, range).into ())
}

#[ allow (clippy::missing_inline_in_public_items) ]
pub fn check_range_error <Val: Debug + Display + Ord, Rng: Debug + RangeBounds <Val>> (
	name: & str,
	val: Val,
	range: Rng,
) -> String {
	match (range.start_bound (), range.end_bound ()) {
		(Bound::Included (start), Bound::Included (end)) =>
			format! ("{name} must be between {start} and {end}, but was {val}"),
		(Bound::Included (start), Bound::Unbounded) =>
			format! ("{name} must be at least {start}, but was {val}"),
		(Bound::Excluded (start), Bound::Unbounded) =>
			format! ("{name} must be more than {start}, but was {val}"),
		(Bound::Unbounded, Bound::Included (end)) =>
			format! ("{name} must be at most {end}, but was {val}"),
		(Bound::Unbounded, Bound::Excluded (end)) =>
			format! ("{name} must be less than {end}, but was {val}"),
		_ =>
			format! ("{name} is out of acceptable range: {range:?}"),
	}
}
