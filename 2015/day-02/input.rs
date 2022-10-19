use super::*;

use model::Dim;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub sizes: Vec <(Dim, Dim, Dim)>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { sizes, params } = [
		params,
		@lines sizes {
			type = (Dim, Dim, Dim);
			(w, h, l) = [ w, "x", h, "x", l ]
		},
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
