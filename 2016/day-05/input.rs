//! Data structures to model the puzzle input

use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub door_id: InpStr <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { door_id, params } = [ params, door_id ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_zeros: u8 = ("NUM_ZEROS=", 5, 1_u8 .. ),
		pub max_threads: u32 = ("MAX_THREADS=", u32::MAX, 1_u32 .. ),
		pub password_len: u8 = ("PASSWORD_LEN=", 8, 1 ..= 8),
		pub batch_size: usize = ("BATCH_SIZE=", 10_000, 1_usize .. ),
	}
}
