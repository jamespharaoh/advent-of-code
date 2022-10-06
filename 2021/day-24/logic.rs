use super::*;

use input::Input;
use machine::Instr;

pub fn part_one (input: & Input) -> GenResult <String> {
	calc_result (& input.instrs, true)
}

pub fn part_two (input: & Input) -> GenResult <String> {
	calc_result (& input.instrs, false)
}

pub fn calc_result (prog: & [Instr], reverse: bool) -> GenResult <String> {
	let steps = quick::steps_for (prog) ?;
	let result = quick::iterator (& steps, reverse).next ().ok_or ("No solution found") ?;
	Ok (model::input_to_str (result))
}
