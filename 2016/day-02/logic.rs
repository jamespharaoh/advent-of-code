use super::*;

use input::Input;
use model::Button;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <String> {
	let digits = |pos| match pos {
		Pos { row: 0, col: 0 } => Some ('1'),
		Pos { row: 0, col: 1 } => Some ('2'),
		Pos { row: 0, col: 2 } => Some ('3'),
		Pos { row: 1, col: 0 } => Some ('4'),
		Pos { row: 1, col: 1 } => Some ('5'),
		Pos { row: 1, col: 2 } => Some ('6'),
		Pos { row: 2, col: 0 } => Some ('7'),
		Pos { row: 2, col: 1 } => Some ('8'),
		Pos { row: 2, col: 2 } => Some ('9'),
		_ => None,
	};
	let code = calc_code (& input.buttons, digits, Pos { row: 1, col: 1 }) ?;
	Ok (code)
}

pub fn part_two (input: & Input) -> GenResult <String> {
	let digits = |pos| match pos {
		Pos { row: 0, col: 2 } => Some ('1'),
		Pos { row: 1, col: 1 } => Some ('2'),
		Pos { row: 1, col: 2 } => Some ('3'),
		Pos { row: 1, col: 3 } => Some ('4'),
		Pos { row: 2, col: 0 } => Some ('5'),
		Pos { row: 2, col: 1 } => Some ('6'),
		Pos { row: 2, col: 2 } => Some ('7'),
		Pos { row: 2, col: 3 } => Some ('8'),
		Pos { row: 2, col: 4 } => Some ('9'),
		Pos { row: 3, col: 1 } => Some ('A'),
		Pos { row: 3, col: 2 } => Some ('B'),
		Pos { row: 3, col: 3 } => Some ('C'),
		Pos { row: 4, col: 2 } => Some ('D'),
		_ => None,
	};
	let code = calc_code (& input.buttons, digits, Pos { row: 2, col: 0 }) ?;
	Ok (code)
}

fn calc_code (
	buttons: & [Button],
	layout_fn: fn (Pos) -> Option <char>,
	mut pos: Pos,
) -> GenResult <String> {
	let mut result = String::new ();
	for button in buttons.iter () {
		for & step in & button.steps {
			let step_one = (step, 1);
			let new_pos = chk! (pos + step_one) ?;
			if layout_fn (new_pos).is_some () { pos = new_pos; }
		}
		result.push (layout_fn (pos).unwrap ());
	}
	Ok (result)
}
