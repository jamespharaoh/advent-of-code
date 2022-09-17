use super::*;

wrapper_deref_mut! {
	#[ derive (Clone, Debug) ]
	pub struct Passport <'inp> {
		fields: Vec <(InpStr <'inp>, InpStr <'inp>)>,
	}
}

impl <'inp> Display for Passport <'inp> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		let mut col = 0;
		for & (ref name, ref value) in & self.fields {
			let name_len = name.chars ().count ();
			let value_len = value.chars ().count ();
			if 0 < col && 80 < col + name_len + value_len + 2 {
				write! (formatter, "\n") ?;
				col = 0;
			} else if 0 < col {
				write! (formatter, " ") ?;
				col += 1;
			}
			write! (formatter, "{name}:{value}") ?;
			col += name_len + value_len + 1;
		}
		write! (formatter, "\n") ?;
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for Passport <'inp> {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		let mut fields = Vec::new ();
		while parser.peek ().unwrap_or ('\n') != '\n' {
			let name = parser.take_rest_while (|ch| ch.is_ascii_alphanumeric (), 1 .. ) ?;
			parser.expect (":") ?;
			let value = parser.take_rest_while (|ch| ! ch.is_whitespace (), 1 .. ) ?;
			fields.push ((name, value));
			if matches! (parser.peek (), Some (' ' | '\n')) { parser.expect_next () ?; }
		}
		Ok (Self { fields })
	}
}
