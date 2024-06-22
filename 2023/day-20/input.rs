use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub modules: Vec <InputModule <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { modules, params } = [ params, @lines modules ]
}

#[ derive (Clone, Debug) ]
pub struct InputModule <'inp> {
	pub type_: InputModuleType,
	pub name: InpStr <'inp>,
	pub dests: Vec <InpStr <'inp>>,
}

impl <'inp> Display for InputModule <'inp> {
	fn fmt (& self, fmtr: & mut fmt::Formatter) -> fmt::Result {
		let Self { type_, ref name, ref dests } = * self;
		match type_ {
			InputModuleType::Broadcast => (),
			InputModuleType::Conjunction => fmtr.write_str ("&") ?,
			InputModuleType::FlipFlop => fmtr.write_str ("%") ?,
		}
		display! (fmtr, name, " -> ", @delim ", " dests);
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for InputModule <'inp> {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parser.any ()
			.of (|parser| {
				parse! (parser, "broadcaster -> ", @delim ", " dests = parse_name);
				let name = InpStr::borrow ("broadcaster");
				Ok (InputModule { type_: InputModuleType::Broadcast, name, dests })
			})
			.of (|parser| {
				parse! (parser, "%", name = parse_name, " -> ", @delim ", " dests = parse_name);
				Ok (InputModule { type_: InputModuleType::FlipFlop, name, dests })
			})
			.of (|parser| {
				parse! (parser, "&", name = parse_name, " -> ", @delim ", " dests = parse_name);
				Ok (InputModule { type_: InputModuleType::Conjunction, name, dests })
			})
			.done ()
	}
}

fn parse_name <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
	parser.take_rest_while (|ch| ch.is_ascii_lowercase (), 1 ..= 6)
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum InputModuleType {
	Conjunction,
	FlipFlop,
	Broadcast,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
