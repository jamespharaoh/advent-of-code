use super::*;

pub type Qty = u64;

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Reaction <'inp> {
	pub inputs: Vec <ChemQty <'inp>>,
	pub output: ChemQty <'inp>,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Reaction <'inp> { inputs, output } = [ @delim_some ", " inputs, " => ", output ]
}

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct ChemQty <'inp> {
	pub chem: InpStr <'inp>,
	pub qty: Qty,
}

struct_display! {
	input_lifetime = 'inp;
	ChemQty <'inp> { chem, qty } = [ qty, " ", chem ]
}

struct_parser! {
	input_lifetime = 'inp;
	ChemQty <'inp> { chem, qty } = [ qty, " ", chem = parse_chem ]
}

fn parse_chem <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
	parser.take_rest_while (|ch| ch.is_ascii_uppercase (), 1 .. )
}
