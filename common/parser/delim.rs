use super::*;

impl <'inp> Parser <'inp> {

	#[ inline ]
	pub fn delim_fn <'par0, Delim, Output, ParseFn> (
		& 'par0 mut self,
		delim: Delim,
		parse_fn: ParseFn,
	) -> ParserDelim <'par0, 'inp, Delim, Output, ParseFn>
		where
			Delim: ParseDelimiter,
			ParseFn: FnMut (& mut Self) -> ParseResult <Output> {
		assert! (delim.is_valid ());
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
			where ParseFn: FnMut (& mut Self) -> ParseResult <Output> {
		ParserRepeat {
			parser: self,
			parse_fn,
		}
	}

	#[ inline ]
	pub fn delim_items <'par, Delim, Output, ParseFn> (
		& 'par mut self,
		delim: Delim,
	) -> ParserDelim <'par, 'inp, Delim, Output, impl FnMut (& mut Parser <'inp>) -> ParseResult <Output>>
		where
			Delim: ParseDelimiter,
			Output: FromParser <'inp> {
		self.delim_fn (delim, Parser::item)
	}

	#[ inline ]
	pub fn delim_uints <'par, Delim, Output: FromStr> (
		& 'par mut self,
		delim: Delim,
	) -> ParserDelim <'par, 'inp, Delim, Output, impl FnMut (& mut Parser <'inp>) -> ParseResult <Output>>
			where Delim: ParseDelimiter {
		self.delim_fn (delim, Parser::uint)
	}

	#[ inline ]
	pub fn delim_ints <'par, Delim, Output: FromStr + 'static> (
		& 'par mut self,
		delim: Delim,
	) -> ParserDelim <'par, 'inp, Delim, Output, impl FnMut (& mut Parser <'inp>) -> ParseResult <Output>>
			where Delim: ParseDelimiter {
		self.delim_fn (delim, Parser::int)
	}

}

pub struct ParserDelim <
	'par,
	'inp,
	Delim,
	Output,
	ParseFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output>,
> {
	parser: & 'par mut Parser <'inp>,
	delim: Delim,
	parse_fn: ParseFn,
	first: bool,
}

impl <'par, 'inp, Delim, Output, ParseFn> Iterator
	for ParserDelim <'par, 'inp, Delim, Output, ParseFn>
	where
		Delim: ParseDelimiter,
		ParseFn: FnMut (& mut Parser <'inp>) -> ParseResult <Output> {

	type Item = Output;

	#[ inline ]
	fn next (& mut self) -> Option <Output> {
		let saved = * self.parser;
		let result = if self.first {
			self.first = false;
			(self.parse_fn) (self.parser)
		} else {
			self.delim.expect (self.parser)
				.and_then (|()| (self.parse_fn) (self.parser))
		};
		if result.is_err () { * self.parser = saved; }
		result.ok ()
	}

}

pub trait ParseDelimiter: Copy + Display + Sized {

	#[ inline ]
	fn is_valid (self) -> bool {
		true
	}

	fn expect (self, parser: & mut Parser) -> ParseResult <()>;

}

impl ParseDelimiter for & str {

	#[ inline ]
	fn is_valid (self) -> bool {
		! self.is_empty ()
	}

	#[ inline ]
	fn expect (self, parser: & mut Parser) -> ParseResult <()> {
		parser.expect (self) ?;
		Ok (())
	}

}

#[ derive (Clone, Copy, Debug) ]
pub struct ParseWhitespace;

impl ParseDelimiter for ParseWhitespace {

	#[ inline ]
	fn expect (self, parser: & mut Parser) -> ParseResult <()> {
		parser.skip_whitespace (1 .. ) ?;
		Ok (())
	}

}

impl Display for ParseWhitespace {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		formatter.write_char (' ')
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
