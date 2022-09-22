use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub earliest: u32,
	pub bus_ids: Vec <Option <u32>>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { earliest, bus_ids, params } = [
		params,
		earliest, "\n",
		@delim "," bus_ids {
			Some (bus_id) = [ bus_id ],
			None = [ "x" ],
		}
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub max_iters: u32 = ("MAX_ITERS=", 1_000, 1_u32 .. ),
	}
}
