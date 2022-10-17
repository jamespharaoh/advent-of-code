use super::*;
use model::Step;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub steps: Vec <Step>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { steps, params } = [ params, @delim "," steps ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub reps_two: u64 = ("REPS_TWO=", 1_000_000_000, 2_u64 .. ),
	}
}
