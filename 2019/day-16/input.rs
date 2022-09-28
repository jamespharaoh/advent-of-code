use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub data: Vec <u8>,
	pub params: InputParams,
}

struct_display! {
	Input { data, params } = [ params, @collect data ]
}

struct_parser! {
	Input { data, params } = [ params, @collect data = parse_num ]
}

fn parse_num (parser: & mut Parser) -> ParseResult <u8> {
	let ch = parser.expect_next () ?;
	if ! ch.is_ascii_digit () { return Err (parser.err ()) }
	Ok (ch.to_digit (10).unwrap ().pan_u8 ())
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_iters: u32 = ("NUM_ITERS=", 100, 1_u32 .. ),
		pub num_reps: u32 = ("NUM_REPS=", 10_000, 1_u32 .. ),
		pub offset_digits: u32 = ("OFFSET_DIGITS=", 7, 1_u32 .. 10_u32),
		pub result_digits: u32 = ("RESULT_DIGITS=", 8, 1_u32 .. 100_u32),
		pub max_signal: u32 = ("MAX_SIGNAL=", 1_000_000, 1_u32 .. ),
	}
}
