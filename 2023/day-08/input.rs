use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub steps: Vec <Step>,
	pub nodes: Vec <Node <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { steps, nodes, params } = [
		params,
		@collect steps, "\n",
		"\n",
		@lines nodes,
	]
}

#[ derive (Clone, Debug) ]
pub struct Node <'inp> {
	pub name: InpStr <'inp>,
	pub left: InpStr <'inp>,
	pub right: InpStr <'inp>,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Node <'inp> { name, left, right } = [
		name = parse_name, " = (",
		left = parse_name, ", ",
		right = parse_name, ")",
	]
}

fn parse_name <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
	parser.take_rest_while (|ch| ch.is_ascii_alphanumeric (), ..= 3)
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Step {
		Left = [ "L" ],
		Right = [ "R" ],
	}
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub max_iters: u64 = ("MAX_ITERS=", 50_000, 1 .. ),
		pub max_steps: u64 = ("MAX_STEPS=", 50_000, 1 .. ),
	}
}
