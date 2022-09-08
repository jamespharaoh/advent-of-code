use super::*;
use model::Pos;
use model::Val;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub depth: Val,
	pub target: Pos,
	pub params: InputParams,
}

struct_parser_display! (Input { depth, target: Pos { y: target_y, x: target_x }, params } = [
	params, "depth: ", depth, "\ntarget: ", target_x, ",", target_y,
]);

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
		pub modulo: u32 = ("MODULO=", 20183, 3_u32 .. ),
		pub top_factor: u32 = ("TOP_FACTOR=", 16_807, 1_u32 .. ),
		pub left_factor: u32 = ("LEFT_FACTOR=", 48_271, 1_u32 .. ),
		pub max_mins: u32 = ("MAX_MINS=", 2000, 1_u32 .. ),
		pub max_target: i16 = ("MAX_TARGET=", 1000, 1_i16 .. ),
	}
}
