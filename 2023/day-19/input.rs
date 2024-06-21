use super::*;

use model::Field;
use model::Op;
use model::Val;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub workflows: Vec <Workflow <'inp>>,
	pub parts: Vec <Part>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { workflows, parts, params } = [
		params,
		@lines workflows,
		"\n\n",
		@lines parts,
	]
}

#[ derive (Clone, Debug) ]
pub struct Workflow <'inp> {
	pub name: InpStr <'inp>,
	pub rules: Vec <Rule <'inp>>,
	pub default: InpStr <'inp>,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Workflow <'inp> { name, rules, default } = [
		name = parse_name, "{", @delim "," rules, ",", default = parse_name, "}"
	]
}

#[ derive (Clone, Debug) ]
pub struct Rule <'inp> {
	pub field: Field,
	pub op: Op,
	pub val: Val,
	pub target: InpStr <'inp>,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Rule <'inp> { field, op, val, target } = [
		field, op, val, ":", target = parse_name,
	]
}

#[ derive (Clone, Debug) ]
pub struct Part {
	pub x: Val,
	pub m: Val,
	pub a: Val,
	pub s: Val,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Part { x, m, a, s } = [ "{x=", x, ",m=", m, ",a=", a, ",s=", s, "}" ]
}

fn parse_name <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
	parser.take_rest_while (|ch| ch.is_ascii_alphabetic (), 1 ..= 3)
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
