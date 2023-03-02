use super::*;

use input::Input;
use model::Grid;
use model::Instr;
use model::Pos;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <i32> {
	iter_reg_x (input).enumerate ()
		.map (|(cycle_idx, reg_x)| (cycle_idx.pan_i32 () + 1_i32, reg_x))
		.skip (19)
		.step_by (40)
		.take (6)
		.map (|(cycle, val)| GenOk (cycle * val ?.pan_i32 ()))
		.try_fold (0_i32, |sum, val| GenOk (chk! (sum + val ?) ?))
}

pub fn part_two (input: & Input) -> GenResult <String> {
	let mut grid = Grid::new_size (Pos::new (6, 40));
	for (pos, reg_x) in
		iter::zip (
			iter_posns (),
			iter_reg_x (input)) {
		let reg_x = reg_x ?;
		if (chk! (reg_x - 1) ? ..= chk! (reg_x + 1) ?).contains (& pos.x) {
			grid.set (pos, true);
		}
	}
	aoc_ocr::read_auto (
		grid.iter ()
			.filter_map (|(pos, val)| val.then_some ((pos.y, pos.x))))
}

fn iter_posns () -> impl Iterator <Item = Pos> {
	let mut pos = Pos::ZERO;
	iter::from_fn (move || {
		if pos.y == 6 { return None }
		let result = pos;
		pos.x += 1;
		if pos.x == 40 {
			pos.y += 1;
			pos.x = 0;
		}
		Some (result)
	})
}

fn iter_reg_x (input: & Input) -> impl Iterator <Item = NumResult <Val>> + '_ {
	let mut reg_x = Val::ONE;
	let mut instr_iter = input.instrs.iter ();
	let mut buffer = VecDeque::new ();
	iter::from_fn (move || {
		loop {
			if let Some (val) = buffer.pop_front () { return Some (Ok (val)) }
			match instr_iter.next ().copied () {
				Some (Instr::AddX (val)) => {
					buffer.push_back (reg_x);
					buffer.push_back (reg_x);
					if chk! (reg_x += val).is_err () { return Some (Err (Overflow)) }
				},
				Some (Instr::Noop) => {
					buffer.push_back (reg_x);
				},
				None => {
					return None;
				},
			}
		}
	})
}
