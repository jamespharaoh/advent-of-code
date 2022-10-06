use super::*;

use model::Amph;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub amphs: [[Amph; 2]; 4],
	pub params: InputParams,
}

struct_parser_display! {
	Input {
		amphs: [ [ a0, a1 ], [ b0, b1 ], [ c0, c1 ], [ d0, d1 ] ],
		params,
	} = [
		params,
		"#############\n",
		"#...........#\n",
		"###", a0, "#", b0, "#", c0, "#", d0, "###\n",
		"  #", a1, "#", b1, "#", c1, "#", d1, "#\n",
		"  #########",
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
