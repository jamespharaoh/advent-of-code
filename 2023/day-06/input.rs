use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub times: Vec <u32>,
	pub distances: Vec <u32>,
	pub params: InputParams,
}

struct_parser_display! {
	Input {
		times,
		distances,
		params,
	} = [
		params,
		"Time: ", @delim " " times { time = [ @skip, time ] }, "\n",
		"Distance: ", @delim " " distances { dist = [ @skip, dist ] },
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
