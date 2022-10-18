//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Instr;
use model::Opcode;
use model::Regs;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let regs: Regs= Regs::default ();
	calc_result (input, regs)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut regs: Regs = Regs::default ();
	regs.set (0, 1).unwrap ();
	calc_result (input, regs)
}

#[ allow (clippy::unneeded_field_pattern) ]
fn calc_result (input: & Input, mut regs: Regs) -> GenResult <u64> {
	let is_sum_of_factors = if input.instrs.len () == 36 {
		matches! (<[Instr; 16]>::try_from (& input.instrs [1 .. 17]).unwrap (), [
			Instr { opcode: Opcode::Seti, arg_a: 1, arg_b: _, arg_c: 2 },
			Instr { opcode: Opcode::Seti, arg_a: 1, arg_b: _, arg_c: 4 },
			Instr { opcode: Opcode::Mulr, arg_a: 2, arg_b: 4, arg_c: 1 },
			Instr { opcode: Opcode::Eqrr, arg_a: 1, arg_b: 5, arg_c: 1 },
			Instr { opcode: Opcode::Addr, arg_a: 1, arg_b: 3, arg_c: 3 },
			Instr { opcode: Opcode::Addi, arg_a: 3, arg_b: 1, arg_c: 3 },
			Instr { opcode: Opcode::Addr, arg_a: 2, arg_b: 0, arg_c: 0 },
			Instr { opcode: Opcode::Addi, arg_a: 4, arg_b: 1, arg_c: 4 },
			Instr { opcode: Opcode::Gtrr, arg_a: 4, arg_b: 5, arg_c: 1 },
			Instr { opcode: Opcode::Addr, arg_a: 3, arg_b: 1, arg_c: 3 },
			Instr { opcode: Opcode::Seti, arg_a: 2, arg_b: _, arg_c: 3 },
			Instr { opcode: Opcode::Addi, arg_a: 2, arg_b: 1, arg_c: 2 },
			Instr { opcode: Opcode::Gtrr, arg_a: 2, arg_b: 5, arg_c: 1 },
			Instr { opcode: Opcode::Addr, arg_a: 1, arg_b: 3, arg_c: 3 },
			Instr { opcode: Opcode::Seti, arg_a: 1, arg_b: _, arg_c: 3 },
			Instr { opcode: Opcode::Mulr, arg_a: 3, arg_b: 3, arg_c: 3 },
		])
	} else { false };
	let mut limit = 10_000_u32;
	#[ allow (clippy::while_let_loop) ]
	loop {
		let instr_val = some_or! (regs.get (input.ip), break);
		let instr_idx = ok_or! (instr_val.to_usize (), break);
		if input.instrs.len () <= instr_idx { break }
		if is_sum_of_factors && instr_idx == 1 {
			let target = regs.get (5).unwrap ();
			return Ok (calc_sum_of_factors (target));
		}
		let instr = input.instrs [instr_idx];
		regs = instr.apply (regs) ?;
		let new_ip = regs.get (input.ip).unwrap () + 1;
		regs.set (input.ip, new_ip).unwrap ();
		limit -= 1;
		if limit == 0 { return Err ("Giving up after 10k instrs".into ()) }
	}
	Ok (regs.get (0).unwrap ())
}

const fn calc_sum_of_factors (target: u64) -> u64 {
	let mut result = target + 1;
	let mut div_0 = 2_u64;
	loop {
		let div_1 = target / div_0;
		let product = div_0 * div_1;
		if product == target {
			if div_0 == div_1 {
				result += div_0;
			} else {
				result += div_0 + div_1;
			}
		}
		if target <= product { break }
		div_0 += 1;
	}
	result
}
