use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub rules: Vec <InputRule>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { rules, params } = [ params, @lines rules ]
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum InputRule {
		TwoToThree (from: u8, to: u16) = [ from = parse_pixels_2, " => ", to = parse_pixels_3 ],
		ThreeToFour (from: u16, to: u16) = [ from = parse_pixels_3, " => ", to = parse_pixels_4 ],
	}
}

pub fn parse_pixels_2 (parser: & mut Parser) -> ParseResult <u8> {
	parse_pixels (parser, 2).map (u64::pan_u8)
}

pub fn parse_pixels_3 (parser: & mut Parser) -> ParseResult <u16> {
	parse_pixels (parser, 3).map (u64::pan_u16)
}

pub fn parse_pixels_4 (parser: & mut Parser) -> ParseResult <u16> {
	parse_pixels (parser, 4).map (u64::pan_u16)
}

pub fn parse_pixels (parser: & mut Parser, size: usize) -> ParseResult <u64> {
	let mut val = 0_u64;
	let new_bit = (1_u64 << ((size * size) - 1)).pan_u64 ();
	for row in 0 .. size {
		if row > 0 { parse! (parser, "/"); }
		for _col in 0 .. size {
			parse! (parser, bit: InputPixel);
			val >>= 1_u32;
			if bit == InputPixel::On { val |= new_bit; }
		}
	}
	Ok (val)
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, PartialEq) ]
	pub enum InputPixel {
		#[ default ]
		Off = [ "." ],
		On = [ "#" ],
	}
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub iters_one: u32 = ("ITERS_ONE=", 5, 1_u32 ..),
		pub iters_two: u32 = ("ITERS_TWO=", 18, 1_u32 ..),
		pub check_rules: bool = ("CHECK_RULES=", true, false ..= true),
	}
}
