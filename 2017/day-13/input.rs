use super::*;

use model::LayerDepth;
use model::LayerRange;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub layers: Vec <Layer>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { layers, params } = [ params, @lines layers ]
}

#[ derive (Clone, Debug, Eq, Ord, PartialEq, PartialOrd) ]
pub struct Layer {
	pub depth: LayerDepth,
	pub range: LayerRange,
}

struct_parser_display! {
	Layer { depth, range } = [ depth, ": ", range ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
