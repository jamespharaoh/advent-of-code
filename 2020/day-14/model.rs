use super::*;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub enum Instr {
	Mask { val: u64, mask: u64 },
	Store { addr: u64, val: u64 },
}

enum_parser_display! {
	Instr,
	Mask { val, mask } = [ "mask", @confirm, " = ", (val, mask) = (mask_parse, mask_display) ],
	Store { addr, val } = [ "mem", @confirm, "[", addr, "] = ", val ],
}

fn mask_parse (parser: & mut Parser) -> ParseResult <(u64, u64)> {
	let mut val = 0;
	let mut mask = 0;
	for _ in 0_u32 .. 36 {
		val <<= 1_u32;
		mask <<= 1_u32;
		match parser.peek () {
			Some ('X') => (),
			Some ('0') => mask |= 1,
			Some ('1') => { val |= 1; mask |= 1; },
			_ => return Err (parser.err ()),
		}
		parser.expect_next ().unwrap ();
	}
	Ok ((val, mask))
}

fn mask_display (
	& (& (mut val), & (mut mask)): & (& u64, & u64),
	formatter: & mut fmt::Formatter,
) -> fmt::Result {
	for _ in 0_u32 .. 36 {
		let mask_bit = mask & (1 << 35_u32) != 0;
		let val_bit = val & (1 << 35_u32) != 0;
		match (mask_bit, val_bit) {
			(false, _) => formatter.write_char ('X') ?,
			(true, false) => formatter.write_char ('0') ?,
			(true, true) => formatter.write_char ('1') ?,
		}
		val <<= 1_u32;
		mask <<= 1_u32;
	}
	Ok (())
}
