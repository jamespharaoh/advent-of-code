use super::*;

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Room <'inp> {
	pub name: InpStr <'inp>,
	pub sector: u32,
	pub checksum: InpStr <'inp>,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Room <'inp> { name, sector, checksum } = [
		name = room_name_parse,
		"-",
		sector,
		"[", @str checksum = (|ch| { ch.is_ascii_lowercase () }, 5 ..= 5), "]",
	]
}

fn room_name_parse <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
	let mut last_ch = None;
	let mut num_chars = 0_u32;
	for ch in parser.peek_rest ().chars () {
		match (last_ch, ch) {
			(Some ('-'), 'a' ..= 'z') => num_chars += 2,
			(_, 'a' ..= 'z') => num_chars += 1,
			(Some ('a' ..= 'z'), '-') => (),
			_ => break,
		}
		last_ch = Some (ch);
	}
	if num_chars < 2 { return Err (parser.err ()) }
	parser.take_exactly (num_chars)
}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn test_room_name_parse () {
		assert_eq_ok! ("abc-def", room_name_parse (& mut Parser::new ("abc-def")));
		assert_eq_ok! ("abc-def", room_name_parse (& mut Parser::new ("abc-def-")));
		assert_eq_ok! ("abc-def", room_name_parse (& mut Parser::new ("abc-def--ghi")));
		assert_eq_ok! ("abc-def", room_name_parse (& mut Parser::new ("abc-def-123")));
		assert_is_err! (room_name_parse (& mut Parser::new ("-abc-def")));
		assert_is_err! (room_name_parse (& mut Parser::new ("BLAH")));
	}

}
