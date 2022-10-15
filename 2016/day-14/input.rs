//! Data structures to model the puzzle input

use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub salt: InpStr <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { salt, params } = [
		params,
		@str salt = (|ch| { ch.is_ascii_lowercase () }, 1 .. ),
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_keys: u32 = ("NUM_KEYS=", 64, 1 ..= 100),
		pub num_next: u32 = ("NUM_NEXT=", 1000, 1 ..= 2000),
		pub hash_reps: u32 = ("HASH_REPS=", 2016, 1 ..= 3000),
		pub max_threads: usize = ("MAX_THREADS=", usize::MAX, 1 .. ),
		pub batch_size: usize = ("BATCH_SIZE=", 1000, 1 .. 10_000),
	}
}
