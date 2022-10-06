use super::*;

use model::Action;
use model::Coord;
use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub steps: Vec <InputStep>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { steps, params } = [ params, @lines steps ]
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub struct InputStep {
	pub action: Action,
	pub origin: Pos,
	pub peak: Pos,
}

struct_parser_display! {
	InputStep {
		action,
		origin: Pos { row: origin_row, col: origin_col },
		peak: Pos { row: peak_row, col: peak_col },
	} = [
		action, " ",
		origin_row = (Coord::MIN .. Coord::MAX), ",", origin_col, " through ",
		peak_row, ",", peak_col,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
