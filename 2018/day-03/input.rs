use super::*;

use model::Coord;
use model::Id;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub claims: Vec <Claim>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { claims, params } = [ params, @lines claims ]
}

#[ derive (Clone, Copy, Debug) ]
pub struct Claim {
	pub id: Id,
	pub left: Coord,
	pub top: Coord,
	pub width: Coord,
	pub height: Coord,
}

struct_parser_display! {
	Claim { id, left, top, width, height } = [
		"#", id, " @ ", left, ",", top, ": ", width, "x", height,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
